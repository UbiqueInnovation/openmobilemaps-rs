/*
 * Copyright (c) 2021 Ubique Innovation AG <https://www.ubique.ch>
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *  SPDX-License-Identifier: MPL-2.0
 */

#include "Polygon2dOpenGl.h"
#include "OpenGlHelper.h"
#include "cxxgen.h"

Polygon2dOpenGl::Polygon2dOpenGl(const std::shared_ptr<::ShaderProgramInterface> &shader)
    : shaderProgram(shader) {}

std::shared_ptr<GraphicsObjectInterface> Polygon2dOpenGl::asGraphicsObject() { return shared_from_this(); }

std::shared_ptr<MaskingObjectInterface> Polygon2dOpenGl::asMaskingObject() { return shared_from_this(); }

bool Polygon2dOpenGl::isReady() { return ready; }

void Polygon2dOpenGl::setVertices(const ::SharedBytes &vertices_, const ::SharedBytes &indices_) {
    std::lock_guard<std::recursive_mutex> lock(dataMutex);
    ready = false;
    dataReady = false;

    indices.resize(indices_.elementCount);
    vertices.resize(vertices_.elementCount);

    if (indices_.elementCount > 0) {
        std::memcpy(indices.data(), (void *)indices_.address, indices_.elementCount * indices_.bytesPerElement);
    }

    if (vertices_.elementCount > 0) {
        std::memcpy(vertices.data(), (void *)vertices_.address, vertices_.elementCount * vertices_.bytesPerElement);
    }

    dataReady = true;
}

void Polygon2dOpenGl::setup(const std::shared_ptr<::RenderingContextInterface> &context) {
    std::lock_guard<std::recursive_mutex> lock(dataMutex);
    if (ready || !dataReady)
        return;

    // log_rs("ready to start\n");
    std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);
    if (openGlContext->getProgram(shaderProgram->getProgramName()) == 0) {
        shaderProgram->setupProgram(openGlContext);
        // log_rs("setupProgram \n");
    }
    programHandle = openGlContext->getProgram(shaderProgram->getProgramName());

    prepareGlData(openGlContext);
    ready = true;
}

void Polygon2dOpenGl::prepareGlData(const std::shared_ptr<OpenGlContext> &openGlContext) {
    glUseProgram(programHandle);
    OpenGlHelper::checkGlError("glUseProgram");

    positionHandle = glGetAttribLocation(programHandle, "vPosition");
    OpenGlHelper::checkGlError("glGetAttribLocation");
    glGenBuffers(1, &vertexBuffer);
    OpenGlHelper::checkGlError("glGenBuffers");
    glBindBuffer(GL_ARRAY_BUFFER, vertexBuffer);
    OpenGlHelper::checkGlError("glBindBuffer");
    glBufferData(GL_ARRAY_BUFFER, sizeof(GLfloat) * vertices.size(), &vertices[0], GL_STATIC_DRAW);
    OpenGlHelper::checkGlError("glBufferData");
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");
    glGenBuffers(1, &indexBuffer);
    OpenGlHelper::checkGlError("glGenBuffers");
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
    OpenGlHelper::checkGlError("glBindBuffer");
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(GLushort) * indices.size(), &indices[0], GL_STATIC_DRAW);
    OpenGlHelper::checkGlError("glBufferData");
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");

    mvpMatrixHandle = glGetUniformLocation(programHandle, "uMVPMatrix");
    OpenGlHelper::checkGlError("glGetUniformLocation");
}

void Polygon2dOpenGl::clear() {
    std::lock_guard<std::recursive_mutex> lock(dataMutex);
    if (ready) {
        removeGlBuffers();
        ready = false;
    }
}

void Polygon2dOpenGl::removeGlBuffers() {
    glDeleteBuffers(1, &vertexBuffer);
    glDeleteBuffers(1, &indexBuffer);
}

void Polygon2dOpenGl::setIsInverseMasked(bool inversed) { isMaskInversed = inversed; }

void Polygon2dOpenGl::render(const std::shared_ptr<::RenderingContextInterface> &context, const RenderPassConfig &renderPass,
                             int64_t mvpMatrix, bool isMasked, double screenPixelAsRealMeterFactor) {
    if (!ready)
        return;

    std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);

    if (isMasked) {
        if (isMaskInversed) {
            glStencilFunc(GL_EQUAL, 0, 255);
            OpenGlHelper::checkGlError("glStencilFunc");
        } else {
            glStencilFunc(GL_EQUAL, 128, 255);
            OpenGlHelper::checkGlError("glStencilFunc");
        }
    }
    glStencilOp(GL_KEEP, GL_KEEP, GL_INCR);
    OpenGlHelper::checkGlError("glStencilOp");
    glColorMask(GL_TRUE, GL_TRUE, GL_TRUE, GL_TRUE);
    OpenGlHelper::checkGlError("glColorMask");

    drawPolygon(openGlContext, programHandle, mvpMatrix);
}

void Polygon2dOpenGl::drawPolygon(std::shared_ptr<OpenGlContext> openGlContext, int program, int64_t mvpMatrix) {
    // Add program to OpenGL environment
    glUseProgram(program);
    OpenGlHelper::checkGlError("glUseProgram");
    shaderProgram->preRender(openGlContext);

    OpenGlHelper::checkGlError("glBindVertexArrayAPPLE");
    glEnableVertexAttribArray(positionHandle);
    OpenGlHelper::checkGlError("glEnableVertexAttribArray");
    glBindBuffer(GL_ARRAY_BUFFER, vertexBuffer);
    OpenGlHelper::checkGlError("glBindBuffer");
    glVertexAttribPointer(positionHandle, 3, GL_FLOAT, false, 0, nullptr);
    OpenGlHelper::checkGlError("glVertexAttribPointer");

    glBindBuffer(GL_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");
    // Apply the projection and view transformation
    glUniformMatrix4fv(mvpMatrixHandle, 1, false, (GLfloat *)mvpMatrix);
    OpenGlHelper::checkGlError("glUniformMatrix4fv");

    // Enable blending
    glEnable(GL_BLEND);
    OpenGlHelper::checkGlError("glEnable(BLEND)");
    glBlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
    OpenGlHelper::checkGlError("glBlendFunc");

    // Draw the triangle
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
    OpenGlHelper::checkGlError("glBindBuffer");
    glDrawElements(GL_TRIANGLES, (unsigned short)indices.size(), GL_UNSIGNED_SHORT, nullptr);
    OpenGlHelper::checkGlError("glDrawElements");

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");

    // Disable vertex array
    glDisableVertexAttribArray(positionHandle);
    OpenGlHelper::checkGlError("glDisableVertexAttribArray");

    glDisable(GL_BLEND);
    OpenGlHelper::checkGlError("glDisable(BLEND)");
}

void Polygon2dOpenGl::renderAsMask(const std::shared_ptr<::RenderingContextInterface> &context,
                                   const ::RenderPassConfig &renderPass, int64_t mvpMatrix, double screenPixelAsRealMeterFactor) {
    if (!ready)
        return;

    std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);

    glColorMask(GL_FALSE, GL_FALSE, GL_FALSE, GL_FALSE);
    OpenGlHelper::checkGlError("glColorMask");
    drawPolygon(openGlContext, programHandle, mvpMatrix);
    glColorMask(GL_TRUE, GL_TRUE, GL_TRUE, GL_TRUE);
    OpenGlHelper::checkGlError("glColorMask");
}

/*
 * Copyright (c) 2021 Ubique Innovation AG <https://www.ubique.ch>
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *  SPDX-License-Identifier: MPL-2.0
 */

#include "Quad2dOpenGl.h"
#include "Logger.h"
#include "OpenGlHelper.h"
#include "TextureHolderInterface.h"

#include "cxxgen.h"
#include <string>

Quad2dOpenGl::Quad2dOpenGl(const std::shared_ptr<::ShaderProgramInterface> &shader)
    : shaderProgram(shader) {}

bool Quad2dOpenGl::isReady() { return ready && (!usesTextureCoords || textureHolder); }

std::shared_ptr<GraphicsObjectInterface> Quad2dOpenGl::asGraphicsObject() { return shared_from_this(); }

std::shared_ptr<MaskingObjectInterface> Quad2dOpenGl::asMaskingObject() { return shared_from_this(); }

void Quad2dOpenGl::clear()
{
    std::lock_guard<std::recursive_mutex> lock(dataMutex);
    if (readyToDraw)
    {
        removeGlBuffers();
    }
    if (textureCoordsReady)
    {
        removeTextureCoordsGlBuffers();
    }
    if (textureHolder)
    {
        removeTexture();
    }
    readyToDraw = false;
    ready = false;
}

void Quad2dOpenGl::setIsInverseMasked(bool inversed) { isMaskInversed = inversed; }

void Quad2dOpenGl::setFrame(const Quad2dD &frame, const RectD &textureCoordinates)
{
    std::lock_guard<std::recursive_mutex> lock(dataMutex);
    readyToDraw = false;
    this->frame = frame;
    this->textureCoordinates = textureCoordinates;
}

void Quad2dOpenGl::setup(const std::shared_ptr<::RenderingContextInterface> &context)
{
    if (readyToDraw)
        return;
    std::lock_guard<std::recursive_mutex> lock(dataMutex);

    float frameZ = 0;
    vertices = {
        (float)frame.topLeft.x,
        (float)frame.topLeft.y,
        frameZ,
        (float)frame.bottomLeft.x,
        (float)frame.bottomLeft.y,
        frameZ,
        (float)frame.bottomRight.x,
        (float)frame.bottomRight.y,
        frameZ,
        (float)frame.topRight.x,
        (float)frame.topRight.y,
        frameZ,
    };
    indices = {
        0,
        1,
        2,
        0,
        2,
        3,
    };
    adjustTextureCoordinates();

    std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);
    if (openGlContext->getProgram(shaderProgram->getProgramName()) == 0)
    {
        shaderProgram->setupProgram(openGlContext);
    }

    int program = openGlContext->getProgram(shaderProgram->getProgramName());
    prepareGlData(openGlContext, program);
    // log_rs("prepare texture data\n");
    prepareTextureCoordsGlData(openGlContext, program);

    programHandle = program;
    // log_rs("Vertices updated\n");
    readyToDraw = true;
}

void Quad2dOpenGl::prepareGlData(const std::shared_ptr<OpenGlContext> &openGlContext, const int &programHandle)
{
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
    glBufferData(GL_ELEMENT_ARRAY_BUFFER, sizeof(GLubyte) * indices.size(), &indices[0], GL_STATIC_DRAW);
    OpenGlHelper::checkGlError("glBufferData");
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");

    mvpMatrixHandle = glGetUniformLocation(programHandle, "uMVPMatrix");
    OpenGlHelper::checkGlError("glGetUniformLocation");
}

void Quad2dOpenGl::prepareTextureCoordsGlData(const std::shared_ptr<OpenGlContext> &openGlContext, const int &programHandle)
{
    glUseProgram(programHandle);
    OpenGlHelper::checkGlError("[Quad2dOpenGl] glUseProgram");
    if (textureCoordsReady)
    {
        removeTextureCoordsGlBuffers();
    }

    textureCoordinateHandle = glGetAttribLocation(programHandle, "texCoordinate");
    OpenGlHelper::checkGlError("glGetAttribLocation");
    if (textureCoordinateHandle < 0)
    {
        usesTextureCoords = false;
        return;
    }
    glGenBuffers(1, &textureCoordsBuffer);
    OpenGlHelper::checkGlError("glGenBuffers");
    glBindBuffer(GL_ARRAY_BUFFER, textureCoordsBuffer);
    OpenGlHelper::checkGlError("glBindBuffer");
    glBufferData(GL_ARRAY_BUFFER, sizeof(GLfloat) * textureCoords.size(), &textureCoords[0], GL_STATIC_DRAW);
    OpenGlHelper::checkGlError("glBufferData");
    glBindBuffer(GL_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");
    // log_rs("texture bound \n");
    usesTextureCoords = true;
    textureCoordsReady = true;
}

void Quad2dOpenGl::removeGlBuffers()
{
    glDeleteBuffers(1, &vertexBuffer);
    OpenGlHelper::checkGlError("glDeleteBuffers");
    glDeleteBuffers(1, &indexBuffer);
    OpenGlHelper::checkGlError("glDeleteBuffers");
}

void Quad2dOpenGl::removeTextureCoordsGlBuffers()
{
    if (textureCoordsReady)
    {
        glDeleteBuffers(1, &textureCoordsBuffer);
        OpenGlHelper::checkGlError("glDeleteBuffers");
        textureCoordsReady = false;
    }
}

void Quad2dOpenGl::loadTexture(const std::shared_ptr<::RenderingContextInterface> &context,
                               const std::shared_ptr<TextureHolderInterface> &textureHolder)
{
    std::lock_guard<std::recursive_mutex> lock(dataMutex);
    if (textureHolder != nullptr)
    {
        texturePointer = textureHolder->attachToGraphics();

        factorHeight = textureHolder->getImageHeight() * 1.0f / textureHolder->getTextureHeight();
        factorWidth = textureHolder->getImageWidth() * 1.0f / textureHolder->getTextureWidth();
        // log_rs(std::to_string(textureHolder->getImageWidth()));
        adjustTextureCoordinates();

        if (ready)
        {
            std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);
            int program = openGlContext->getProgram(shaderProgram->getProgramName());
            prepareTextureCoordsGlData(openGlContext, program);
        }
        // log_rs("Texture loaded");
        this->textureHolder = textureHolder;
    }
}

void Quad2dOpenGl::removeTexture()
{
    std::lock_guard<std::recursive_mutex> lock(dataMutex);
    if (textureHolder)
    {
        textureHolder->clearFromGraphics();
        textureHolder = nullptr;
        texturePointer = -1;
        if (textureCoordsReady)
        {
            removeTextureCoordsGlBuffers();
        }
    }
}

void Quad2dOpenGl::adjustTextureCoordinates()
{
    float tMinX = factorWidth * textureCoordinates.x;
    float tMaxX = factorWidth * (textureCoordinates.x + textureCoordinates.width);
    float tMinY = factorHeight * textureCoordinates.y;
    float tMaxY = factorHeight * (textureCoordinates.y + textureCoordinates.height);

    textureCoords = {tMinX, tMinY, tMinX, tMaxY, tMaxX, tMaxY, tMaxX, tMinY};
}

void Quad2dOpenGl::renderAsMask(const std::shared_ptr<::RenderingContextInterface> &context, const RenderPassConfig &renderPass,
                                int64_t mvpMatrix, double screenPixelAsRealMeterFactor)
{
    glColorMask(GL_FALSE, GL_FALSE, GL_FALSE, GL_FALSE);
    OpenGlHelper::checkGlError("glColorMask");
    render(context, renderPass, mvpMatrix, false, screenPixelAsRealMeterFactor);
    glColorMask(GL_TRUE, GL_TRUE, GL_TRUE, GL_TRUE);
    OpenGlHelper::checkGlError("glColorMask");
}

void Quad2dOpenGl::render(const std::shared_ptr<::RenderingContextInterface> &context, const RenderPassConfig &renderPass,
                          int64_t mvpMatrix, bool isMasked, double screenPixelAsRealMeterFactor)
{
    // log_rs("IN RENDER");
    if (!readyToDraw || (usesTextureCoords && !textureCoordsReady))
    {
        return;
    }

    glUseProgram(programHandle);
    OpenGlHelper::checkGlError("glUseProgram");

    if (isMasked)
    {
        glStencilFunc(GL_EQUAL, isMaskInversed ? 0 : 128, 128);
        OpenGlHelper::checkGlError("glStencilFunc");
        glStencilOp(GL_KEEP, GL_KEEP, GL_KEEP);
        OpenGlHelper::checkGlError("glStencilOp");
    }

    std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);
    int mProgram = openGlContext->getProgram(shaderProgram->getProgramName());
    glUseProgram(mProgram);
    OpenGlHelper::checkGlError("glUseProgram");

    if (usesTextureCoords)
    {
        prepareTextureDraw(openGlContext, programHandle);

        glEnableVertexAttribArray(textureCoordinateHandle);
        OpenGlHelper::checkGlError("glEnableVertexAttribArray");
        glBindBuffer(GL_ARRAY_BUFFER, textureCoordsBuffer);
        OpenGlHelper::checkGlError("glBindBuffer");
        glVertexAttribPointer(textureCoordinateHandle, 2, GL_FLOAT, false, 0, nullptr);
        OpenGlHelper::checkGlError("glVertexAttribPointer");
    }

    shaderProgram->preRender(context);

    // enable vPosition attribs
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
    OpenGlHelper::checkGlError("glEnable");
    glBlendFunc(GL_ONE, GL_ONE_MINUS_SRC_ALPHA);
    OpenGlHelper::checkGlError("glBlendFunc");

    // Draw the triangles
    // log_rs("draw elements");
    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, indexBuffer);
    OpenGlHelper::checkGlError("glBindBuffer");
    glDrawElements(GL_TRIANGLES, 6, GL_UNSIGNED_BYTE, nullptr);
    OpenGlHelper::checkGlError("glDrawElements");

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");

    glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, 0);
    OpenGlHelper::checkGlError("glBindBuffer");

    // Disable vertex array
    glDisableVertexAttribArray(positionHandle);
    OpenGlHelper::checkGlError("glDisableVertexAttribArray");

    if (textureHolder)
    {
        glDisableVertexAttribArray(textureCoordinateHandle);
        OpenGlHelper::checkGlError("glDisableVertexAttribArray");
    }

    glDisable(GL_BLEND);
    OpenGlHelper::checkGlError("glDisable");
    ready = true;
}

void Quad2dOpenGl::prepareTextureDraw(std::shared_ptr<OpenGlContext> &openGLContext, int programHandle)
{
    if (!textureHolder)
    {
        log_rs("no texture holder");
        return;
    }

    // Set the active texture unit to texture unit 0.
    glActiveTexture(GL_TEXTURE0);
    OpenGlHelper::checkGlError("glActiveTexture");

    // Bind the texture to this unit.
    glBindTexture(GL_TEXTURE_2D, (unsigned int)texturePointer);
    OpenGlHelper::checkGlError("glBindTexture");

    // Tell the texture uniform sampler to use this texture in the shader by binding to texture unit 0.
    int textureUniformHandle = glGetUniformLocation(programHandle, "sampler");
    OpenGlHelper::checkGlError("glGetUniformLocation");
    glUniform1i(textureUniformHandle, 0);
    OpenGlHelper::checkGlError("glUniform1i");
}

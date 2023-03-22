/*
 * Copyright (c) 2021 Ubique Innovation AG <https://www.ubique.ch>
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *  SPDX-License-Identifier: MPL-2.0
 */

#include "ColorShaderOpenGl.h"
#include "OpenGlContext.h"
#include "OpenGlHelper.h"

std::string ColorShaderOpenGl::getProgramName() { return "UBMAP_ColorShaderOpenGl"; }

void ColorShaderOpenGl::setupProgram(const std::shared_ptr<::RenderingContextInterface> &context) {
    std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);
    std::string programName = getProgramName();
    // prepare shaders and OpenGL program
    int vertexShader = loadShader(GL_VERTEX_SHADER, getVertexShader());
    int fragmentShader = loadShader(GL_FRAGMENT_SHADER, getFragmentShader());

    int program = glCreateProgram();       // create empty OpenGL Program
    OpenGlHelper::checkGlError("glCreateProgram");
    glAttachShader(program, vertexShader); // add the vertex shader to program
    OpenGlHelper::checkGlError("glAttachShader");
    glDeleteShader(vertexShader);
    OpenGlHelper::checkGlError("glDeleteShader");
    glAttachShader(program, fragmentShader); // add the fragment shader to program
    OpenGlHelper::checkGlError("glAttachShader");
    glDeleteShader(fragmentShader);
    OpenGlHelper::checkGlError("glDeleteShader");

    glLinkProgram(program); // create OpenGL program executables
    OpenGlHelper::checkGlError("glLinkProgram");

    // log_rs(std::string("Store Program: ") + programName);
    openGlContext->storeProgram(programName, program);
}

void ColorShaderOpenGl::preRender(const std::shared_ptr<::RenderingContextInterface> &context) {
    std::shared_ptr<OpenGlContext> openGlContext = std::static_pointer_cast<OpenGlContext>(context);
    int program = openGlContext->getProgram(getProgramName());

    int mColorHandle = glGetUniformLocation(program, "vColor");
    OpenGlHelper::checkGlError("glGetUniformLocation");
    // OpenGlHelper::checkGlError("glGetUniformLocation");
    glUniform4fv(mColorHandle, 1, &color[0]);
    OpenGlHelper::checkGlError("glUniform4fv");
}

void ColorShaderOpenGl::setColor(float red, float green, float blue, float alpha) {
    color = std::vector<float>{red, green, blue, alpha};
}

std::string ColorShaderOpenGl::getVertexShader() {
    return UBRendererShaderCode(#version 330 \n

                                    precision highp float;
                                uniform mat4 uMVPMatrix; in vec4 vPosition;

                                void main() { gl_Position = uMVPMatrix * vPosition; });
}

std::string ColorShaderOpenGl::getFragmentShader() {
    return UBRendererShaderCode(#version 330  \n

                                    precision mediump float;
                                uniform vec4 vColor; out vec4 fragmentColor;

                                void main() {
                                    fragmentColor = vColor;
                                    fragmentColor.a = 1.0;
                                    fragmentColor *= vColor.a;
                                });
}

std::shared_ptr<ShaderProgramInterface> ColorShaderOpenGl::asShaderProgramInterface() { return shared_from_this(); }

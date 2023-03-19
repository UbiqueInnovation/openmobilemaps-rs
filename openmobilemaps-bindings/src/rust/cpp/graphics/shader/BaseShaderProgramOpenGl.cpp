/*
 * Copyright (c) 2021 Ubique Innovation AG <https://www.ubique.ch>
 *
 *  This Source Code Form is subject to the terms of the Mozilla Public
 *  License, v. 2.0. If a copy of the MPL was not distributed with this
 *  file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 *  SPDX-License-Identifier: MPL-2.0
 */

#include "BaseShaderProgramOpenGl.h"
#include "OpenGlHelper.h"
#include "cxxgen.h"
#include <vector>

int BaseShaderProgramOpenGl::loadShader(int type, std::string shaderCode) {
    // create a vertex shader type (GLES20.GL_VERTEX_SHADER)
    // or a fragment shader type (GLES20.GL_FRAGMENT_SHADER)
    int shader = glCreateShader(type);
    log_rs("Compiling Shader\n");

    // LogInfo << "Compiling Shader Code: " << shaderCode <<= "\n\n";

    // add the source code to the shader and compile it
    log_rs("--> 1\n");
    const char *code = shaderCode.c_str();
    log_rs("--> 2\n");
    int code_length = int(shaderCode.size());
    log_rs("--> 3\n");
    glShaderSource(shader, 1, &code, &code_length);
    OpenGlHelper::checkGlError("glShaderSource");
    glCompileShader(shader);
    OpenGlHelper::checkGlError("glCompileShader");

    GLint isCompiled = 0;
    glGetShaderiv(shader, GL_COMPILE_STATUS, &isCompiled);
    OpenGlHelper::checkGlError("glGetShaderiv");
    if (isCompiled == GL_FALSE) {
        log_rs("compiling failed\n ");
        GLint maxLength = 0;
        glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &maxLength);
        OpenGlHelper::checkGlError("glGetShaderiv");

        // The maxLength includes the NULL character
        std::vector<GLchar> errorLog(maxLength);
        glGetShaderInfoLog(shader, maxLength, &maxLength, &errorLog[0]);
        OpenGlHelper::checkGlError("glGetShaderInfoLog");

       
        auto compiler_error = "";
        // LogInfo << "test" <<= "blib";
        LogInfo << "Shader " << shader << " failed:\n" <<= "";

        for (auto a : errorLog) {
            compiler_error += a;
            LogInfo << a <<= "";
        }
        // log_rs(compiler_error);
        LogInfo <<= ".\n\n";
    } else {
        LogInfo << "Compiling SUCCEEDED: " << shader <<= "\n\n";
    }

    return shader;
}

void BaseShaderProgramOpenGl::checkGlProgramLinking(GLuint program) {
    GLint isLinked = 0;

    glGetProgramiv(program, GL_LINK_STATUS, &isLinked);
    OpenGlHelper::checkGlError("glGetProgramiv");
    if (isLinked == GL_FALSE) {
        GLint maxLength = 0;
        glGetProgramiv(program, GL_INFO_LOG_LENGTH, &maxLength);
        OpenGlHelper::checkGlError("glGetProgramiv");

        // The maxLength includes the NULL character
        std::vector<GLchar> infoLog(maxLength);
        glGetProgramInfoLog(program, maxLength, &maxLength, &infoLog[0]);
        OpenGlHelper::checkGlError("glGetProgramInfoLog");

        log_rs("linking failed");
        LogError << "OpenGL Program Linking failed:";
        auto info_log = "";
        for (auto a : infoLog) {
            LogError << a;
            info_log += a;
        }
        log_rs(info_log);

        LogError <<= ".";
    }
}

std::string BaseShaderProgramOpenGl::getVertexShader() {
    return UBRendererShaderCode(#version 330 \n uniform mat4 uMVPMatrix; in vec4 vPosition; in vec2 texCoordinate;
                                out vec2 v_texcoord;

                                void main() {
                                    gl_Position = uMVPMatrix * vPosition;
                                    v_texcoord = texCoordinate;
                                });
}

std::string BaseShaderProgramOpenGl::getFragmentShader() {
    return UBRendererShaderCode(#version 330 \n precision mediump float; uniform sampler2D sampler; in vec2 v_texcoord;
                                out vec4 fragmentColor;

                                void main() { fragmentColor = texture(sampler, v_texcoord); });
}

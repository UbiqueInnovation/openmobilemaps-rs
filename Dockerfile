FROM  rust:1.67-bullseye as builder
RUN apt update
RUN apt upgrade -y
# RUN echo "deb http://ftp.de.debian.org/debian sid main" >> /etc/apt/sources.list



# RUN apt remove -y usrmerge


RUN git clone https://github.com/rui314/mold.git \
    && mkdir mold/build \
    && cd mold/build \
    && git checkout v1.11.0 \
    && ../install-build-deps.sh \
    && cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ .. \
    && cmake --build . -j $(nproc) \ 
    && cmake --install .

RUN apt remove -y gcc

RUN apt install -y clang llvm
RUN update-alternatives --config c++
RUN apt install -y freeglut3-dev libclang-dev

# RUN apt install -y ca-certificates fonts-liberation libasound2 libatk-bridge2.0-0 libatk1.0-0 libc6 libcairo2 libcups2 libdbus-1-3 libexpat1 libfontconfig1 libgbm1 libgcc1 libglib2.0-0 libgtk-3-0 libnspr4 libnss3 libpango-1.0-0 libpangocairo-1.0-0 libstdc++6 libx11-6 libx11-xcb1 libxcb1 libxcomposite1 libxcursor1 libxdamage1 libxext6 libxfixes3 libxi6 libxrandr2 libxrender1 libxss1 libxtst6 lsb-release wget xdg-utils
# RUN apt install -y mutter dbus-x11 mesa-utils

# ENV XDG_RUNTIME_DIR /app/runtime
# RUN dbus-launch --auto-syntax > dbus-env

# RUN mutter --headless --virtual-monitor 1920x1080 &
# ENV DISPLAY :0
# ENV WAYLAND_DISPLAY wayland-0


WORKDIR openmobilemaps
ADD ./maps-core maps-core
ADD ./cxx cxx
ADD ./openmobilemaps-bindings openmobilemaps-bindings
ADD ./openmobilemaps-sys openmobilemaps-sys
ADD ./src src
ADD ./AvertaStd-Bold.ttf AvertaStd-Bold.ttf 
ADD ./bottomstuff.jpeg bottomstuff.jpeg
ADD ./Cargo.lock Cargo.lock
ADD ./Cargo.toml Cargo.toml
ADD ./assets assets

RUN mold -run cargo install --path=.

FROM ubuntu:latest
RUN apt update
# RUN apt install -y ca-certificates fonts-liberation libasound2 libatk-bridge2.0-0 libatk1.0-0 libc6 libcairo2 libcups2 libdbus-1-3 libexpat1 libfontconfig1 libgbm1 libgcc1 libglib2.0-0 libgtk-3-0 libnspr4 libnss3 libpango-1.0-0 libpangocairo-1.0-0 libstdc++6 libx11-6 libx11-xcb1 libxcb1 libxcomposite1 libxcursor1 libxdamage1 libxext6 libxfixes3 libxi6 libxrandr2 libxrender1 libxss1 libxtst6 lsb-release wget xdg-utils
RUN apt install -y mesa-utils freeglut3-dev libgl1-mesa-glx libglapi-mesa libfontconfig1
# RUN apt install -y  sway sudo
# RUN apt install -y xwayland

# RUN echo 'openmobilemaps ALL = NOPASSWD: /bin/Xvfb' >> /etc/sudoers
# RUN chmod +s /bin/Xvfb
RUN groupadd -r openmobilemaps && useradd -r -g openmobilemaps openmobilemaps
RUN mkhomedir_helper openmobilemaps


COPY --from=builder /usr/local/cargo/bin/openmobilemaps-rs /usr/local/bin

# ENV DISPLAY :99

ENV LIBGL_ALWAYS_SOFTWARE true
ENV LIBGL_ALWAYS_INDIRECT true
ENV LIBGL_DEBUG verbose
ENV MESA_DEBUG verbose,incomplete_tex,incomplete_fbo,context
# ENV XDG_RUNTIME_DIR /app/runtime
# ENV WLR_BACKENDS headless
# ENV WLR_LIBINPUT_NO_DEVICES 1
# ENV WAYLAND_DISPLAY wayland-1
# ENV WLR_RENDERER_ALLOW_SOFTWARE true
# ENV XDG_RUNTIME_DIR /home/openmobilemaps/runtime
# ENV DISPLAY :99
# ENV XDG_SESSION_TYPE wayland
ENV MESA_SHADER_CACHE_DISABLE true
ENV MESA_GL_VERSION_OVERRIDE 4.5COMPAT
ENV MESA_GLSL_VERSION_OVERRIDE 450
ENV force_glsl_extensions_warn true

ADD ./entrypoint.sh /home/openmobilemaps/entrypoint.sh
RUN chmod +x /home/openmobilemaps/entrypoint.sh

USER openmobilemaps
RUN mkdir /home/openmobilemaps/runtime
WORKDIR /home/openmobilemaps
ADD ./AvertaStd-Bold.ttf AvertaStd-Bold.ttf 
ADD ./bottomstuff.jpeg bottomstuff.jpeg
ADD ./train.png train.png
ADD ./assets assets
RUN mkdir output

ENTRYPOINT [ "./entrypoint.sh"]
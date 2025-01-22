FROM mcr.microsoft.com/devcontainers/rust:bookworm

RUN cargo install create-tauri-app --locked

# Prepare the environment
RUN apt update

# Install Node.js 23.x
RUN curl -fsSL https://deb.nodesource.com/setup_23.x -o nodesource_setup.sh
RUN sudo -E bash nodesource_setup.sh
RUN apt install -y nodejs
RUN rm nodesource_setup.sh

# Install tauri dependencies
RUN apt install -y libwebkit2gtk-4.1-dev \
    build-essential \
    curl \
    wget \
    file \
    libxdo-dev \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev

# Install IPFS
RUN apt install -y ipfs
RUN ipfs init
RUN ipfs config --json API.HTTPHeaders.Access-Control-Allow-Origin '["*"]'

# Install dependencies
RUN npm install

# Add user to render group
RUN groupadd -g 992 render
RUN usermod -a -G render root

EXPOSE 1420

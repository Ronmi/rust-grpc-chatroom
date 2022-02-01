FROM rustembedded/cross:x86_64-pc-windows-gnu-0.2.1

RUN apt update && apt install -y libssl-dev

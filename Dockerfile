FROM julia:1.7-buster 

RUN apt update \
    && apt install -y python3 python3-pip \
    && python3 -m pip install julia \
    && python3 -c "import julia ; julia.install()" 

SHELL [ "/bin/bash" ] 

VOLUME [ "/app" ] 
WORKDIR "/app" 

ENTRYPOINT [ "python-jl" ] 



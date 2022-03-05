FROM ubuntu:20.04

RUN apt-get update && apt-get install git python3 python3-pip python3.8-venv -y \
    && git clone https://github.com/shijiameng/HARM-rust \
    && pip3 install pyelftools capstone keystone-engine
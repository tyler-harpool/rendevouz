#!/bin/sh
docker build -t rendevouz -f $PWD/containers/local/Rendevouz.dev.Dockerfile $PWD  && docker run -p 8000:8000 rendevouz

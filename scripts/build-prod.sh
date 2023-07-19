#!/bin/sh
docker build -t rendevouz -f containers/prod/Rendevouz.prod.Dockerfile $PWD && docker run -p 8000:8000 rendevouz
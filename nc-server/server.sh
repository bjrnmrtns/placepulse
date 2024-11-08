#!/usr/bin/env bash
while true; do
  { echo -e "HTTP/1.1 200 OK\r\nContent-Length: 50\r\nContent-Type: text/html\r\n\r\n<html><body><h1>Hello from Netcat!</h1></body></html"; } | nc -l 8080
done

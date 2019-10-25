#!/bin/sh

GRPC_PY_DIR=py
python3 -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR --grpc_python_out=$GRPC_PY_DIR core.proto
python3 -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR cartesi-base.proto
python3 -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR --grpc_python_out=$GRPC_PY_DIR manager-low.proto
python3 -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR --grpc_python_out=$GRPC_PY_DIR manager-high.proto
python3 -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR --grpc_python_out=$GRPC_PY_DIR logger-high.proto




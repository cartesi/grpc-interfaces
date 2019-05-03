#!/bin/bash

GRPC_PY_DIR=py
python -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR --grpc_python_out=$GRPC_PY_DIR core.proto
python -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR cartesi-base.proto
python -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR --grpc_python_out=$GRPC_PY_DIR manager-low.proto
python -m grpc_tools.protoc  -I. --python_out=$GRPC_PY_DIR --grpc_python_out=$GRPC_PY_DIR manager-high.proto




# -*- coding: utf-8 -*-
# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: d2.proto
# Protobuf Python Version: 5.26.1
"""Generated protocol buffer code."""
from google.protobuf import descriptor as _descriptor
from google.protobuf import descriptor_pool as _descriptor_pool
from google.protobuf import symbol_database as _symbol_database
from google.protobuf.internal import builder as _builder
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor_pool.Default().AddSerializedFile(b'\n\x08\x64\x32.proto\x12\x05\x64\x32rpc\"\x1c\n\x04Vec2\x12\t\n\x01x\x18\x01 \x01(\x02\x12\t\n\x01y\x18\x02 \x01(\x02\"\x89\x01\n\x0e\x41xesDescriptor\x12\x1b\n\x06origin\x18\x01 \x01(\x0b\x32\x0b.d2rpc.Vec2\x12\x1b\n\x06x_axis\x18\x02 \x01(\x0b\x32\x0b.d2rpc.Vec2\x12\x1b\n\x06y_axis\x18\x03 \x01(\x0b\x32\x0b.d2rpc.Vec2\x12\x0f\n\x07neg_len\x18\x04 \x01(\x02\x12\x0f\n\x07pos_len\x18\x05 \x01(\x02\"@\n\x0e\x41\x64\x64\x41xesRequest\x12.\n\x0f\x61xes_descriptor\x18\x01 \x01(\x0b\x32\x15.d2rpc.AxesDescriptor\"\"\n\x0f\x41\x64\x64\x41xesResponse\x12\x0f\n\x07\x61xes_id\x18\x01 \x01(\x04\x32H\n\x0cStateService\x12\x38\n\x07\x41\x64\x64\x41xes\x12\x15.d2rpc.AddAxesRequest\x1a\x16.d2rpc.AddAxesResponseb\x06proto3')

_globals = globals()
_builder.BuildMessageAndEnumDescriptors(DESCRIPTOR, _globals)
_builder.BuildTopDescriptorsAndMessages(DESCRIPTOR, 'd2_pb2', _globals)
if not _descriptor._USE_C_DESCRIPTORS:
  DESCRIPTOR._loaded_options = None
  _globals['_VEC2']._serialized_start=19
  _globals['_VEC2']._serialized_end=47
  _globals['_AXESDESCRIPTOR']._serialized_start=50
  _globals['_AXESDESCRIPTOR']._serialized_end=187
  _globals['_ADDAXESREQUEST']._serialized_start=189
  _globals['_ADDAXESREQUEST']._serialized_end=253
  _globals['_ADDAXESRESPONSE']._serialized_start=255
  _globals['_ADDAXESRESPONSE']._serialized_end=289
  _globals['_STATESERVICE']._serialized_start=291
  _globals['_STATESERVICE']._serialized_end=363
# @@protoc_insertion_point(module_scope)

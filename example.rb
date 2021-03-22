require 'ffi'

module LibConfium
  extend FFI::Library
  ffi_lib 'confium'

  attach_function :cfm_create,
                  %i[pointer],
                  :uint32
  attach_function :cfm_destroy,
                  %i[pointer],
                  :uint32
=begin
  attach_function :cfm_hash_create,
                  %i[string pointer],
                  :uint32
  attach_function :cfm_hash_destroy,
                  %i[pointer],
                  :uint32
=end
  attach_function :cfm_plugin_load,
                  %i[pointer string pointer pointer],
                  :uint32
end

pptr = FFI::MemoryPointer.new(:pointer)
LibConfium.cfm_create(pptr)
lib = pptr.read_pointer

puts LibConfium.cfm_plugin_load(lib, 'libhash_botan.so', nil, nil)

#LibConfium.cfm_hash_create('SHA256', pptr)
#hash = pptr.read_pointer
#LibConfium.cfm_hash_destroy(hash)

LibConfium.cfm_destroy(lib)


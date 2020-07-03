cp deps/CascLib/src/CascLib.h deps/CascLib/src/CascLib.hpp && \
bindgen deps/CascLib/src/CascLib.hpp -o casclib-sys/src/bindings.rs \
  --whitelist-function "^Casc.+" \
  --whitelist-type "^CASC.+" \
  --whitelist-var "^CASC.+" &&\
  rm deps/CascLib/src/CascLib.hpp
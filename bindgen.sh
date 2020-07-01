cp casclib-sys/CascLib/src/CascLib.h casclib-sys/CascLib/src/CascLib.hpp && \
bindgen casclib-sys/CascLib/src/CascLib.hpp -o bindings.rs \
  --whitelist-function "^Casc.+" \
  --blacklist-function "SetLastError" \
  --whitelist-type "^CASC.+" &&\
  rm casclib-sys/CascLib/src/CascLib.hpp
# Example EVE Filetype Plugin

## Building

If in the Suricata source directory, this plugin can be built by
running `make` and installed with `make install`.

Note that Suricata must have been built without `--disable-shared`.

## Building Standalone

The file `Makefile.example` is an example of how you might build a
plugin that is distributed separately from the Suricata source code.

It has the following dependencies:

- Suricata is installed
- The Suricata library is installed: `make install-library`
- The Suricata development headers are installed: `make install-headers`
- The program `libsuricata-config` is in your path (installed with
  `make install-library`)

The run: `make -f Makefile.example`

Before building this plugin you will need to build and install Suricata from the
git master branch and install the development tools and headers:

- `make install-library`
- `make install-headers`

then make sure the newly installed tool `libsuricata-config` can be
found in your path, for example:
```
libsuricata-config --cflags
```

Then a simple `make` should build this plugin.

Or if the Suricata installation is not in the path, a command like the following
can be used:

```
PATH=/opt/suricata/bin:$PATH make
```

## Usage

To run the plugin, first add the path to the plugin you just compiled to
your `suricata.yaml`, for example:
```
plugins:
  - /usr/lib/suricata/plugins/json-filetype.so
```

Then add an output for the plugin:
```
outputs:
  - eve-log:
      enabled: yes
      filetype: json-filetype-plugin
      threaded: true
      types:
        - dns
        - tls
        - http
```

In the example above we use the name specified in the plugin as the `filetype`
and specify that all `dns`, `tls` and `http` log entries should be sent to the
plugin.

## Details

This plugin demonstrates a Suricata JSON/EVE output plugin
(file-type). The idea of a Suricata EVE output plugin is to provide a
file like interface for the handling of rendered JSON logs. This is
useful for custom destinations not builtin to Suricata or if the
formatted JSON requires some post-processing.

Note: EVE output plugins are not that useful just for reformatting the
JSON output as the plugin does need to handle writing to a file once
the file type has been delegated to the plugin.

### Registering a Plugin

All Suricata plugins make themselves known to Suricata by using a
function named `SCPluginRegister` which is called after Suricata loads
the plugin shared object file. This function must return a `SCPlugin`
struct which contains basic information about the plugin.  For
example:

```c
const SCPlugin PluginRegistration = {
    .name = "eve-filetype",
    .author = "Jason Ish",
    .license = "GPLv2",
    .Init = TemplateInit,
};

const SCPlugin *SCPluginRegister() {
    return &PluginRegistration;
}
```

### Initializing a Plugin

After the plugin has been registered, the `Init` callback will be called. This
is where the plugin will set itself up as a specific type of plugin such as an
EVE output, or a capture method.

This plugins registers itself as an EVE file type using the
`SCRegisterEveFileType` struct. To register as an EVE file type the
following must be provided:

* name: This is the name of the output which will be used in the eve filetype
  field in `suricata.yaml` to enable this output.
* Init: The callback called when the output is "opened".
* Deinit: The callback called the output is "closed".
* ThreadInit: Callback called to initialize per thread data (if threaded).
* ThreadDeinit: Callback called to deinitialize per thread data (if threaded).
* Write: The callback called when an EVE record is to be "written".

Please see the code in `filetype.c` for more details about this functions.

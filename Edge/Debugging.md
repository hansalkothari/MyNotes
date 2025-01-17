# DEBUGGING WITH GSTREAMER

## Method 1 : Running the pipeline with debugging logs
- Key Debug Levels:
    GST_DEBUG=2: General warnings and errors.
    GST_DEBUG=3: Information messages (useful for debugging).
    GST_DEBUG=4: Detailed messages for plugins.
    GST_DEBUG_DUMP_DOT_DIR: Dumps a dot file of the pipeline for visualization.

> GST_DEBUG_DUMP_DOT_DIR ?

## Method 2 : Ensuring appsrc is pushing buffers correctly
```
GstElement *appsrc = gst_bin_get_by_name(GST_BIN(pipeline), "appsrc");
GstCaps *caps = gst_caps_from_string("video/x-bayer,format=bggr,width=5472,height=3468");
g_object_set(appsrc, "caps", caps, NULL);
gst_caps_unref(caps);

```
> caps?

## use gst-inspect-1.0

for reading about a particular plugin.
```
gst-inspect-1.0 bayer2rgb

```
Test individual pipeline segments

```
gst-launch-1.0 appsrc name=appsrc ! video/x-bayer,format=bggr,width=5472,height=3468 ! bayer2rgb ! fakesink
```
```
gst-launch-1.0 videotestsrc ! nvvideoconvert ! video/x-raw(memory:NVMM),format=RGBA ! fakesink
```

## Debug with gst logs

If you suspect specific plugins are causing the issue, focus on them by enabling logs for those elements only:

```
GST_DEBUG=nvvideoconvert:4,bayer2rgb:3 gst-launch-1.0 [pipeline]
```
**Analyzing the logs**
- element initialization

    * logs about creating and linking `0:00:01.001123456 12345 0xabcdef INFO                GST_INIT gst.c:527:init_thread: Initialized GStreamer 1.18.3`

- caps negotiation 
    - look for messages like `0:00:02.456123456 12345 0xabcdef DEBUG               GST_CAPS gstpad.c:3000:gst_pad_set_caps:<nvvideoconvert0:src> caps = video/x-raw(memory:NVMM), format=RGBA, width=5472, height=3468`

- warning with errors

- memory issue

**Debugging steps**
1. Check for caps mismatch `WARNING: could not negotiate caps`.
2. Locate broken links `ERROR: from element /GstPipeline:pipeline0/GstElement:appsrc0: Could not link appsrc to bayer2rgb.`
3. Inspect plugin specific log `GST_DEBUG=nvvideoconvert:5 gst-launch-1.0 [pipeline]`
4. Trace memory and buffers with level 4 or level 5.

**save logs for analysis**
GST_DEBUG=4 gst-launch-1.0 [pipeline] > debug_logs.txt 2>&1


## Generating a dot graph
If you can't analyze the error from the logs, generate a dot graph.
- Enable dot graph generation `GST_DEBUG_DUMP_DOT_DIR=/path/to/dot-files gst-launch-1.0 [pipeline] `.
- Locate the generated .dot files (e.g., pipeline.STATE_PLAYING.dot)
- Visualize them using Graphviz
    `dot -Tpng /path/to/dot-files/pipeline.STATE_PLAYING.dot -o pipeline.png`

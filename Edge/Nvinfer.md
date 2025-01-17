# How to use nvinfer inside the deepstream pipleine?


> Deepstream uses TensorRT for inference.


## prerequisits:
    - Keep the .onnx model ready.
    - Ensure you know the input/output tensor names and their dimensions (use tools like Netron to inspect the model).
    - Install TensorRT onnx parser.

## what does config file do in deepstream? What should it contain? For which plugins is it necessary?

## steps:
1. DeepStream requires a TensorRT engine file (.engine). Use the trtexec tool to generate this file.
```
trtexec --onnx=yolov10.onnx --saveEngine=yolov10.engine --explicitBatch
```
2. Verify the .engine file is generated.
3. Configure the *nvinfer* plugin.
```
[primary-gie]
enable=1
gpu-id=0
batch-size=1
model-engine-file=path/to/yolov10.engine
labelfile-path=path/to/labels.txt
network-mode=0
num-detected-classes=80
gie-unique-id=1
process-mode=1
network-type=0
config-file-path=path/to/yolov10_config.txt

```
4. configure the model specific **yolov10_config.txt**
```
# YOLOv10 Config File
net-scale-factor=0.00392
model-color-format=0
custom-network-config=path/to/yolov10.cfg
model-file=path/to/yolov10.weights
input-dims=1;3;640;640;  # Adjust based on your model
uff-input-blob-name=input
output-blob-names=boxes;scores
num-detected-classes=80

```
5. In the deepstream_app_config.txt file:
```
[source0]
enable=1
type=3
uri=path/to/video.mp4

[sink0]
enable=1
type=3
container=1
codec=1
sync=0

```
6. Run the deepstream application.
`deepstream-app -c deepstream_app_config.txt
`

## Accessing inference results programmatically
**Using probe function**

Attaching a probe to *nvosd* or other plugin's pads.

```
GstPad *osd_sink_pad = gst_element_get_static_pad(osd, "sink");
if (!osd_sink_pad) {
    g_printerr("Unable to get sink pad\n");
} else {
    gst_pad_add_probe(osd_sink_pad, GST_PAD_PROBE_TYPE_BUFFER, osd_sink_pad_buffer_probe, NULL, NULL);
}
```
**Define the probe function to access metadata**
```
static GstPadProbeReturn osd_sink_pad_buffer_probe(GstPad *pad, GstPadProbeInfo *info, gpointer u_data) {
    GstBuffer *buf = (GstBuffer *)info->data;

    NvDsBatchMeta *batch_meta = gst_buffer_get_nvds_batch_meta(buf);
    if (!batch_meta) return GST_PAD_PROBE_OK;

    for (NvDsMetaList *l_frame = batch_meta->frame_meta_list; l_frame != NULL; l_frame = l_frame->next) {
        NvDsFrameMeta *frame_meta = (NvDsFrameMeta *)(l_frame->data);
        for (NvDsMetaList *l_obj = frame_meta->obj_meta_list; l_obj != NULL; l_obj = l_obj->next) {
            NvDsObjectMeta *obj_meta = (NvDsObjectMeta *)(l_obj->data);
            g_print("Object Class: %d, Confidence: %f, BBox: (%f, %f, %f, %f)\n",
                    obj_meta->class_id, obj_meta->confidence,
                    obj_meta->rect_params.left, obj_meta->rect_params.top,
                    obj_meta->rect_params.width, obj_meta->rect_params.height);
        }
    }
    return GST_PAD_PROBE_OK;
}
```

## nvinfer + Bytetrack
nvinfer takes the frames and run the inference to generate detections. These detections are stored in the NvDsBatchMeta and NvDsObjectMeta metadata.
The metadata contains : 
- Frame Meta (NvDsFrameMeta): Information about the video frame.
- Object Meta (NvDsObjectMeta): Object detection results, including:
    * Class ID.
    * Confidence score.
    * Bounding box coordinates.
    * Unique tracking IDs (if a tracker is used)

## What to do after using *nvinfer* plugin?

> Does the shared memory stores both the inference results and the frames?
> What is a probe function.
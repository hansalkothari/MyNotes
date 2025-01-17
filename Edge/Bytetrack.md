# ALGORITHM

!["bytetrack algorithm"](/assets/image.png)

The input to the BYTE algorithm is a video sequence along the Detector. Also a detection threshold value. The algorithm outputs Tracks T of the video each frame contains the bounding box and the ID of the objects.

For each frame in the video, first we predict the detection boxes and prediction score using the Detector Det. Then we separate the detection boxes between Det(high) and Det(low) according to the detection score threshold.

After separating the detection boxes, the Kalman filter is applied to predict the new location in the current frame of each Track T. Firstly association is applied on high detection boxes after that left-over low detection boxes association will be applied.

> ByteTrack algorithm takes in low scores detection also (with high scores detections) in consideration for object tracking.

## What is a Kalman Filter?
Use the **Kalman Filter and IoU-based Hungarian Matching algorithm** to associate *high-confidence detections with existing tracks*.

- Predict the position of existing tracks using a Kalman Filter.
- Compute the IoU (Intersection over Union) between predicted tracks and high-confidence detections.
- Perform association using the Hungarian algorithm:
- Assign detections to tracks based on maximum IoU.
- Unmatched tracks are marked as lost.
- Unmatched detections are treated as new tracks.

> **IOU** : 

> **Hungarian Algorithm**

> **Shared Library**

## How will we use Bytetrack in our as a custom plugin in our deepstream pipline?



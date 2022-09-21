class RegisterTextureResponse {
  final int textureId;

  RegisterTextureResponse.fromMap(Map map)
      : textureId = map["texture_id"] as int;
  // videoTexturePointer = map["video_texture_ptr"] as int,
  // updateFrameCallbackPointer = map["update_frame_callback_ptr"] as int;
}

class DeregisterTextureRequest {
  final int textureId;
  // final int videoTexturePointer;

  DeregisterTextureRequest(
    this.textureId,
    /*this.videoTexturePointer*/
  );

  Map toMap() => {
        "texture_id": textureId,
        // "video_texture_ptr": videoTexturePointer,
      };
}

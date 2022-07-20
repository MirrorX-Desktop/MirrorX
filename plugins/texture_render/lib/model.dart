class RegisterTextureResponse {
  final int textureID;
  final int videoTexturePointer;
  final int updateFrameCallbackPointer;

  RegisterTextureResponse.fromMap(Map map)
      : textureID = map["texture_id"] as int,
        videoTexturePointer = map["video_texture_ptr"] as int,
        updateFrameCallbackPointer = map["update_frame_callback_ptr"] as int;
}

class DeregisterTextureRequest {
  final int textureID;
  final int videoTexturePointer;

  DeregisterTextureRequest(this.textureID, this.videoTexturePointer);

  Map toMap() => {
        "texture_id": textureID,
        "video_texture_ptr": videoTexturePointer,
      };
}

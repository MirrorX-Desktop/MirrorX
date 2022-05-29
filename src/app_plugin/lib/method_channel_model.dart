class RegisterVideoTextureModel {
  final int textureID;
  final int videoTexturePointer;
  final int updateFrameCallbackPointer;

  RegisterVideoTextureModel.fromMap(Map map)
      : textureID = map["texture_id"] as int,
        videoTexturePointer = map["video_texture_ptr"] as int,
        updateFrameCallbackPointer = map["update_frame_callback_ptr"] as int;
}

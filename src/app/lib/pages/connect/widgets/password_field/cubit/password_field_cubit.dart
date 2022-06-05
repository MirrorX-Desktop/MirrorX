import 'package:bloc/bloc.dart';
import 'package:meta/meta.dart';

part 'password_field_state.dart';

class PasswordFieldCubit extends Cubit<PasswordFieldState> {
  PasswordFieldCubit() : super(PasswordFieldInitial());
}

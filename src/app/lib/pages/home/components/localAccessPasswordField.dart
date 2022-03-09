import 'package:easy_localization/easy_localization.dart';
import 'package:flutter/material.dart';

class LocalAccessPasswordField extends StatefulWidget {
  const LocalAccessPasswordField({Key? key}) : super(key: key);

  @override
  _LocalAccessPasswordFieldState createState() =>
      _LocalAccessPasswordFieldState();
}

class _LocalAccessPasswordFieldState extends State<LocalAccessPasswordField> {
  bool _passwordVisable = false;
  final String _password = "45678213";

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: 40,
      child: Row(children: [
        Expanded(
          child: Center(
            child: Text(
              _passwordVisable ? _password : "＊＊＊＊＊＊",
              style: const TextStyle(
                fontSize: 16,
              ),
            ),
          ),
        ),
        IconButton(
          tooltip: _passwordVisable
              ? tr("connect_to_remote.local_access_password_hide")
              : tr("connect_to_remote.local_access_password_show"),
          onPressed: () => setState(() {
            _passwordVisable = !_passwordVisable;
          }),
          splashRadius: 14,
          splashColor: Colors.transparent,
          hoverColor: const Color.fromARGB(240, 220, 220, 220),
          icon: Icon(
            _passwordVisable ? Icons.visibility_off : Icons.visibility,
            size: 16,
          ),
        ),
        IconButton(
          tooltip: tr("connect_to_remote.local_access_password_edit"),
          onPressed: () {},
          splashRadius: 14,
          splashColor: Colors.transparent,
          hoverColor: const Color.fromARGB(240, 220, 220, 220),
          icon: const Icon(
            Icons.edit,
            size: 16,
          ),
        ),
      ]),
    );
  }
}

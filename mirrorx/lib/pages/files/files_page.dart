import 'package:flutter/material.dart';
import 'package:mirrorx/pages/desktop/widgets/desktop_render_box/desktop_render_box.dart';

class FilesPage extends StatelessWidget {
  const FilesPage({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    // return const Center(child: Text("Files Transfer is comming soon!"));
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            Tooltip(
              preferBelow: true,
              message: "Raw Resolution",
              child: Container(
                width: 36,
                height: 36,
                padding: const EdgeInsets.all(3.0),
                child: TextButton(
                  onPressed: () {},
                  style: ButtonStyle(
                    shape: MaterialStateProperty.all(
                      RoundedRectangleBorder(
                          borderRadius: BorderRadius.circular(4.0)),
                    ),
                    padding: MaterialStateProperty.all(EdgeInsets.zero),
                    foregroundColor: MaterialStateProperty.all(Colors.black),
                  ),
                  child: const Icon(Icons.fit_screen),
                ),
              ),
            ),
          ],
        ),
        Expanded(
          child: Container(
            color: Colors.black,
          ),
        )
      ],
    );
  }
}

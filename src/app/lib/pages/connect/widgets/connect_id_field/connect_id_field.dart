import 'package:flutter/material.dart';

class ConnectIdField extends StatelessWidget {
  const ConnectIdField({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        border: Border(left: BorderSide(color: Colors.yellow, width: 4)),
      ),
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 12.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              "Device ID",
              style: TextStyle(fontSize: 27),
            ),
            Text(
              "11-523-663",
              style: TextStyle(fontSize: 45),
            ),
          ],
        ),
      ),
    );
  }
}

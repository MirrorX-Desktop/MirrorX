import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:mirrorx/state/profile/profile_state_cubit.dart';

const languages = {
  "en": "English",
  "zh": "简体中文",
};

class LanguageSelector extends StatelessWidget {
  const LanguageSelector({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Text(
          AppLocalizations.of(context)!.settingsPageOptionLanguageTitle,
          style: const TextStyle(fontSize: 28),
        ),
        DropdownButtonHideUnderline(
          child: DropdownButton<Locale>(
            elevation: 1,
            underline: null,
            dropdownColor: Colors.white,
            focusColor: Colors.transparent,
            borderRadius: const BorderRadius.all(Radius.circular(8)),
            value: Localizations.localeOf(context),
            items: AppLocalizations.supportedLocales
                .map((item) => DropdownMenuItem(
                      value: item,
                      child: Text(languages[item.languageCode]!),
                    ))
                .toList(),
            onChanged: (Locale? value) =>
                context.read<ProfileStateCubit>().changeLocale(value),
          ),
        ),
      ],
    );
  }
}

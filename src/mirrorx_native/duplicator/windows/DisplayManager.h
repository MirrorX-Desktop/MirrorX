#ifndef DISPLAY_MANAGER_H
#define DISPLAY_MANAGER_H

#include "DisplayOutput.h"
#include "common.h"

using namespace std;

class DisplayManager {
public:
  DisplayManager();
  ~DisplayManager();
  std::vector<DisplayOutput *> ListDisplays();

private:
  std::vector<DisplayOutput *> m_displays;
};

#endif // DISPLAY_MANAGER_H

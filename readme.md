пакет tch - обертка rust над С++ библиотекой libtorch.  крайне не стабилен в плане совместимости версий libtorch и cuda toolkit,
ниже ссылки на окружение cuda которое взлетело таки на windows 10
libtorch: 
https://download.pytorch.org/libtorch/cu124/libtorch-win-shared-with-deps-debug-2.4.0%2Bcu124.zip
Распакуйте арпхив в локальную папку. например в C:\\tools\libtorch. для совместимости с tch версии 0.17.0 важно скачать версию libtorch именно с зависимостями 2.4.0 по ссылке выше.
На сайте pytorch на данный момент предлагается скачивать архив с версией deps 2.5.1, она компилироваться не будет. 
Установите путь к папке в переменную окружения %LIBTORCH%
добавьте  путь %LIBTORCH%\lib в переменную окружения PATH

rucstup default должен возвращать msvc toolchain


Для включения вычисления на видеокарте с поддержкой cuda скачайте и установите cuda toolkit.
https://developer.download.nvidia.com/compute/cuda/12.4.1/network_installers/cuda_12.4.1_windows_network.exe

в версии tch 0.18.0 не работает cuda, это баг tch, так как нативная программа на C++ c libtorch deps 2.5.1 компилируется и поддерживает cuda.
поэтому, если хотим считать нейронки на видеокарте ставим вресию tch 0.17.0

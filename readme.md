tch
  - Пакет tch - обертка rust над С++ библиотекой libtorch.  крайне не стабилен в плане совместимости версий libtorch и cuda toolkit, и они сами меж собой и с cl компилятором msvc тоже конфликутую.
  - Ниже ссылки на окружение cuda которое взлетело таки на windows 10.

visual studio 2022
  - Установите инструменты сборки С++ для linux.
  
msvc
  - Окружение rustup default должено возвращать msvc toolchain, типа stable-x86_64-pc-windows-msvc (default).

libtorch 
  - https://download.pytorch.org/libtorch/cu124/libtorch-win-shared-with-deps-debug-2.4.0%2Bcu124.zip
  - Распакуйте арпхив в локальную папку. например в C:\\tools\libtorch. для совместимости с tch версии 0.17.0 важно скачать версию libtorch именно с зависимостями 2.4.0 по ссылке выше.
  - На сайте pytorch на данный момент предлагается скачивать архив с версией deps 2.5.1, она с tch v0.17.0 компилироваться не будет. 
  - Установите путь к папке в переменную окружения %LIBTORCH%.
  - Добавьте  путь %LIBTORCH%\lib\ в переменную окружения PATH


  
cuda
  - Для включения вычисления на видеокарте с поддержкой cuda скачайте и установите cuda toolkit.
  - https://developer.download.nvidia.com/compute/cuda/12.4.1/network_installers/cuda_12.4.1_windows_network.exe
  - В версии tch 0.18.0 cuda не работает! это баг tch, так как нативная программа на C++ c libtorch deps 2.5.1 компилируется и поддерживает cuda.
  - Поэтому, если хотим считать нейронки на видеокарте в rust, ставим версию tch 0.17.0.

wsl
 -в Windows окружении работа не стабильна! Программа может упасть с ошибкой сегментайии памяти.
 - рекомендуется отлака и сборка в окружении wsl (ubuntu)

   wsl автоматичеки пробрасывает видеокарту для работы с Cuda. при этом в нем доступна VSCode для разработки.
 - установите дистрибутив linux (ubuntu) в wsl.
 - для работу нужно установить pytorch v2.4.0

    pip install torch==2.4.0 torchvision==0.19.0 torchaudio==2.4.0  --index-url https://download.pytorch.org/whl/cu124 --break-system-packages
 - libtorch 
 
    https://download.pytorch.org/libtorch/cu124/libtorch-shared-with-deps-2.4.0%2Bcu124.zip
    Распакуйте в $HOME/dev/libtorch
  - cuda

    sudo apt install nvidia-cuda-toolkit
  
 - переменные окружения:
   -  export LIBTORCH=$HOME/dev/libtorch
   -  export LD_LIBRARY_PATH=$LIBTORCH/lib
   -  export PATH=$LD_LIBRARY_PATH:$PATH
   -  export LIBTORCH_USE_PYTORCH=1
 - 



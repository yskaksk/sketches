# sketches

## ギャラリー

### 2022/05/18

![](./gallery/20220518.gif)

### 2022/05/19

![](./gallery/20220519_2.png)

### 2022/05/24

![](./gallery/20220524.png)

### 2022/05/31

![](./gallery/20220531.gif)

### 2022/06/05

![](./gallery/20220605_2.gif)

### 2022/06/06

![](./gallery/20220606.png)

### 2022/06/08

![](./gallery/20220608.png)

### 2022/06/27

![](./gallery/20220627.png)

### 2022/06/28

![](./gallery/20220628.gif)

### 2022/07/01

![](./gallery/20220701.png)

## Gifの作り方

```bash
$ ffmpeg -i image_%03d.png -filter_complex "[0:v] fps=5,scale=1000:-1,split [a][b];[a] palettegen [p];[b][p] paletteuse" output.gif
```

* `scale=1000:-1` はwidthを1000pxにして、heightはアスペクト比率を維持する

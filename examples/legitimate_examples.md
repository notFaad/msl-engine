# Legitimate MSL Engine Use Cases

## 1. Public Image Galleries
```msl
open "https://unsplash.com/s/photos/nature"

wait 3

media
  image
    where src ~ "images.unsplash.com"
    extensions jpg, jpeg, png

save to "./nature_photos"
```

## 2. News Site Images
```msl
open "https://www.bbc.com/news"

wait 2

media
  image
    where src ~ "ichef.bbci.co.uk"
    extensions jpg, jpeg, png

save to "./news_images"
```

## 3. Educational Content
```msl
open "https://en.wikipedia.org/wiki/Art"

wait 3

media
  image
    where src ~ "upload.wikimedia.org"
    extensions jpg, jpeg, png

save to "./art_images"
```

## 4. Public APIs
```msl
open "https://jsonplaceholder.typicode.com/photos"

media
  image
    extensions jpg, jpeg, png

save to "./api_images"
```

## 5. Stock Photo Sites
```msl
open "https://picsum.photos/"

wait 2

media
  image
    where src ~ "picsum.photos"
    extensions jpg, jpeg, png

save to "./stock_photos"
``` 
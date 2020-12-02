# Maintainer: Sam Belliveau <sam.belliveau@gmail.com>
pkgname=markov_chain-bin
pkgver=0.2.0
pkgrel=1
pkgdesc="a utility for creating markov chains based on dictionaries"
url="https://github.com/Sam-Belliveau/Markov-Chain"
license=("GPL")
arch=("x86_64")
provides=("markov_chain")
options=("strip")
source=("https://github.com/Sam-Belliveau/Markov-Chain/releases/download/v$pkgver/markov_chain-$pkgver-x86_64.tar.gz")
sha256sums=("ebe75d5a5529fcb87f8d557bd6af4359a09a3a8c7b679f5ef1df0a03fe1a861a")

package() {
    install -Dm755 markov_chain -t "$pkgdir/usr/bin/"
}

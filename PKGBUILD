pkgname=kbind-bin
pkgver=1.0.0
pkgrel=1
pkgdesc="Natural language shell command generator"
arch=('x86_64')
url="https://github.com/zpeteman/kbind"
license=('MIT')
provides=('kbind')
conflicts=('kbind')
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/zpeteman/kbind/releases/download/v${pkgver}/kbind.pkg.tar.zst")
sha256sums=('SKIP')

package() {
  # The ZST package from nfpm already contains the exact filesystem structure (e.g. /usr/bin/kbind)
  # So we just extract it to pkgdir
  tar -I zstd -xf "${pkgname}-${pkgver}.tar.gz" -C "$pkgdir"
}

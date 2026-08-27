pkgname=nlsh-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Natural language shell command generator"
arch=('x86_64')
url="https://github.com/zpeteman/nlsh"
license=('MIT')
provides=('nlsh')
conflicts=('nlsh')
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/zpeteman/nlsh/releases/download/v${pkgver}/nlsh.pkg.tar.zst")
sha256sums=('SKIP')

package() {
  # The ZST package from nfpm already contains the exact filesystem structure (e.g. /usr/bin/nlsh)
  # So we just extract it to pkgdir
  tar -I zstd -xf "${pkgname}-${pkgver}.tar.gz" -C "$pkgdir"
}

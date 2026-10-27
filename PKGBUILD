# PKGBUILD for elfinstaller

pkgname=elfinstaller
pkgver=0.1.0
pkgrel=1
pkgdesc="A simple terminal application to install and manage ELF packages"
arch=('x86_64' 'aarch64')
url="https://github.com/andy64lol/elfinstaller"
license=('MIT')
makedepends=('rust')
source=("${pkgname}-${pkgver}.tar.gz::https://github.com/andy64lol/elfinstaller/archive/refs/tags/v${pkgver}.tar.gz")
sha256sums=('SKIP')
options=('!debug')

build() {
    cd "${pkgname}-${pkgver}"
    cargo build --release --locked
}

package() {
    cd "${pkgname}-${pkgver}"
    install -Dm755 target/release/elfinstaller "${pkgdir}/usr/bin/elfinstaller"
    install -Dm644 LICENSE "${pkgdir}/usr/share/licenses/${pkgname}/LICENSE"
    install -Dm644 README.md "${pkgdir}/usr/share/doc/${pkgname}/README.md"
}

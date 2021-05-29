# Maintainer: David Westen <davidlwesten@protonmail.com>
pkgname='bytes'
pkgver=r4.2f6dc19
pkgrel=1
pkgdesc="view bytes of a file on stdout"
arch=('x86_64')
makedepends=('git' 'cargo')

pkgver() {
	printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

build() {
	return 0
}

package() {
	cd ..
	cargo install --path . --root "$pkgdir/usr/" --no-track
}

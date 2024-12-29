{ lib, ... }:
{
  description = "Your trusty omnibox search.";
  homepage = "https://github.com/friedow/centerpiece";
  mainProgram = "centerpiece";
  platforms = lib.platforms.linux;
  license = [ lib.licenses.mit ];
  maintainers = [ "friedow" ];
}

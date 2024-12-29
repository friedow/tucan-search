{ self, ... }:
{
  GIT_DATE = "${builtins.substring 0 4 self.lastModifiedDate}-${
    builtins.substring 4 2 self.lastModifiedDate
  }-${builtins.substring 6 2 self.lastModifiedDate}";
  GIT_REV = self.shortRev or self.dirtyShortRev;
}

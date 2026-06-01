from phermes_build import firstboot as fb


def test_write_firstboot_flag_creates_file(tmp_path):
    fb.write_firstboot_flag(str(tmp_path))
    assert (tmp_path / "firstboot.flag").exists()


def test_firstboot_flag_content(tmp_path):
    fb.write_firstboot_flag(str(tmp_path))
    content = (tmp_path / "firstboot.flag").read_text()
    assert "pending" in content


def test_write_motd_contains_phermes(tmp_path):
    fb.write_motd(str(tmp_path), hostname="phermes", ip_hint="<your-ip>")
    issue = tmp_path / "etc" / "issue"
    assert issue.exists()
    content = issue.read_text()
    assert "PHermes" in content
    assert "phermes.local" in content


def test_write_motd_contains_url(tmp_path):
    fb.write_motd(str(tmp_path), hostname="phermes", ip_hint="192.168.1.x")
    content = (tmp_path / "etc" / "issue").read_text()
    assert "https://" in content

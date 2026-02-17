# 🧧 Lunar New Year Wish Generator

Một công cụ CLI tuyệt vời để tạo lời chúc Tết Nguyên Đán một cách nhanh chóng và dễ dàng!

## 📋 Giới Thiệu

**Lunar New Year Wish Generator** là một ứng dụng dòng lệnh (TUI) được xây dựng bằng Rust, giúp bạn:

- ✨ Tạo lời chúc Tết personalized cho các đối tượng khác nhau
- 🎓 Hỗ trợ chúc cho: Giáo viên, Anh chị (Tiền bối), Bạn bè, và Đàn em
- 🎯 Chọn từ nhiều loại lời chúc khác nhau (Học tập, Nhan sắc, Tình duyên, Tiền bạc, Code/IT,...)
- 📋 Giao diện user-friendly với TUI tương tác
- 📋 Tự động copy lời chúc vào clipboard

## 🚀 Cách Sử Dụng

### 1. Download từ GitHub Actions

Vào repo: **[new_year_wish_gen_tui](https://github.com/vuthanhtrung2010/new_year_wish_gen_tui)**

Các bước tải xuống:
1. Vào trang [**Actions**](https://github.com/vuthanhtrung2010/new_year_wish_gen_tui/actions) tab
2. Chọn phiên bản phù hợp với hệ điều hành của bạn:
   - **Windows**: `happy_new_year.exe` (cho Windows x64/ARM64)
   - **Linux**: `happy_new_year` (cho Linux x64/ARM64)
3. Download file và extract

### 2. Chạy ứng dụng

#### Windows:
```cmd
happy_new_year.exe
```

#### Linux/macOS:
```bash
# Cho quyền thực thi
chmod +x happy_new_year

# Chạy
./happy_new_year
```

### 3. Sử Dụng

1. Chọn người nhận lời chúc (Giáo viên, Anh/Chị, Bạn bè, Đàn em)
2. Nhập tên và thông tin cần thiết
3. Chọn các loại lời chúc mong muốn
4. Nhận lời chúc hoàn thành
5. Lời chúc sẽ được **tự động copy** vào clipboard của bạn.

## 🔧 Yêu Cầu Hệ Thống

### Windows:
- Windows 7 trở lên

### Linux:
- Một trong các công cụ clipboard sau phải được cài đặt:
  - **X11**: `xclip` hoặc `xsel`
  - **Wayland**: `wl-copy`
  
  Cài đặt trên Ubuntu/Debian:
  ```bash
  sudo apt-get install xclip
  # hoặc
  sudo apt-get install wl-clipboard
  ```

### macOS:
- macOS 10.5+
- Tự build lại từ source code (xem phần dưới)

## 💾 Build từ Source

Nếu bạn muốn build từ source code:

```bash
git clone https://github.com/vuthanhtrung2010/new_year_wish_gen_tui.git
cd new_year_wish_gen_tui

cargo build --release # nhớ cài Rust toolchain trc đó

./target/release/happy_new_year
```

## 🐛 Troubleshooting

### "Clipboard tool not found" (Linux)
Cài đặt công cụ clipboard:
```bash
sudo apt-get install xclip
```

### Chương trình không chạy
- Đảm bảo bạn đã download đúng phiên bản cho OS của mình
- Kiểm tra quyền thực thi (Linux/macOS): `chmod +x happy_new_year`

## 🤝 Đóng Góp

Nếu bạn có ý tưởng hoặc phát hiện bug, vui lòng tạo Issues hoặc Pull Requests trên GitHub!

**Chúc bạn một năm mới bình an :D**
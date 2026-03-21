from src.victoria_download import download_metrics

from datetime import datetime, timedelta

def main():
    end = datetime(2026, 3, 21, 16, 9, 0)
    
    df = download_metrics(
        start=end - timedelta(hours=1),
        end=end,
    )

    print(df)

if __name__ == "__main__":
    main()

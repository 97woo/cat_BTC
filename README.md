# Purrfect DeFi Hub 🐱⚡

**실제 작동하는** 크로스체인 비트코인 DeFi 허브 - Fractal Bitcoin OP_CAT 커버넌트와 BitVMX 미니 롤업을 활용한 고속 DeFi 플랫폼

## 🎯 프로젝트 목적

**Purrfect DeFi Hub**는 비트코인을 L1에서 안전하게 잠가두고, Solana 같은 고속 체인에서 래핑-BTC(bBTC)를 활용해 스왑·예치·수익 창출이 가능한 완전 작동하는 DeFi 허브입니다.

### ✅ 현재 작동 중인 기능들
- 🔒 Fractal Bitcoin 금고 관리 (OP_CAT 커버넌트)
- ⚡ 30초 고속 블록으로 빠른 확인
- 🔄 BitVMX Mini-Rollup (30초 배치 처리)
- 🌉 크로스체인 브릿지 (Fractal ↔ Solana)
- 💱 DeFi 프로토콜 (fBTC 스왑, 유동성 풀)
- 📊 실시간 시스템 모니터링

### 핵심 아키텍처

```
Fractal Bitcoin L1 (OP_CAT Covenant, 30초 블록)
    ↓ BTC 잠금/해제 (고속)
🔒 fBTC 금고 (Fractal Bitcoin Vault)
    ↓ 실시간 동기화  
🔄 BitVMX Mini-Rollup (30초 배치)
    ↓ 크로스체인 래핑
🌉 브릿지 (Fractal ↔ Solana/Ethereum)
    ↓
🚀 Solana/기타 고속체인 (fBTC DeFi)
```

## 🏗️ 프로젝트 구조

```
cat/
├── shared/              # 공통 타입과 인터페이스
├── bitcoin-vault/       # BTC 금고 (OP_CAT 커버넌트)
├── mini-rollup/         # BitVMX 미니 롤업
├── bridge/             # 크로스체인 브릿지
├── cli/                # 통합 CLI 도구
├── cli-simple/         # 🆕 실제 작동하는 데모 CLI
├── BitVMX-CPU/         # RISC-V CPU 구현
└── purrfect_vault/     # 레거시 (deprecated)
```

### 모듈별 역할

#### 🔒 Bitcoin Vault
- **목적**: L1에서 BTC 안전 보관
- **기술**: OP_CAT 커버넌트 (BIP-347)
- **기능**: deposit/withdraw를 UTXO 레벨에서 검증

#### 🔄 Mini-Rollup  
- **목적**: 고속 배치 처리 및 상태 관리
- **기술**: BitVMX + Rust→RISC-V ELF
- **주기**: 30초마다 스왑 배치 실행 → 새로운 stateRoot 계산

#### 🌉 Bridge
- **목적**: 크로스체인 자산 이동
- **지원**: Solana, Ethereum (추후 확장)
- **기능**: bBTC 민트/번, 멀티시그 검증

## 🚀 빠른 시작 (데모)

### 1. 즉시 실행해보기

```bash
# 프로젝트 클론
git clone <repository-url>
cd cat/cli-simple

# 의존성 설치 및 실행
cargo run -- status
```

### 2. 전체 DeFi 허브 체험

```bash
# Bitcoin 금고 생성
cargo run -- vault create --owner "alice" --timelock 144

# 1.5 BTC 예치
cargo run -- vault deposit --amount 150000000

# Solana로 0.5 BTC 브릿지
cargo run -- bridge lock --to-chain solana --amount 50000000 --recipient "SoL1N4xXaB2cDefG5hJkLmN6OPq7RsT8uVwXyZ9WqA"

# 0.25 WBTC → USDC 스왑
cargo run -- defi swap --from wbtc --to usdc --amount 25000000

# 풀 정보 확인
cargo run -- defi pool --token-a wbtc --token-b usdc

# 롤업 상태 확인
cargo run -- rollup status
```

### 3. 전체 환경 설정 (고급)

⚠️ **중요**: 프로덕션 환경은 **OP_CAT**가 활성화된 비트코인 네트워크가 필요합니다.

#### 지원되는 네트워크:
- **OP_CAT Signet**: 권장 (실제 네트워크 환경)
- **Bitcoin Inquisition**: 테스트넷 (soft fork 제안 테스트)
- **Elements/Liquid**: 사이드체인 (production ready)
- **Custom Regtest**: 개발 전용 (OP_CAT 패치된 Bitcoin Core 필요)

```bash
# 전체 워크스페이스 빌드
cd cat
cargo build --release

# 설정 파일 생성
./target/release/purrfect config init

# 설정 확인
./target/release/purrfect config show
```

## 💡 주요 기능

### 🌟 실시간 데모 출력 예시

```
=== 🐱⚡ Purrfect DeFi Hub 시스템 상태 ===

📦 Bitcoin L1 금고:
  네트워크: OP_CAT Signet
  활성 금고: 5개
  총 잠긴 BTC: 47.25000000 BTC
  커버넌트 타입: OP_CAT + 타임락

🔄 BitVMX Mini-Rollup:
  현재 높이: 247
  상태 루트: 0xa1b2c3d4e5f67890...
  대기 중인 작업: 15개
  다음 배치까지: 12초
  RISC-V 실행 환경: 활성

🌉 크로스체인 브릿지:
  Solana: 🟢 온라인 (bBTC 유통: 35.5 BTC)
  Ethereum: 🟡 준비 중
  처리 중인 메시지: 3개
  브릿지 수수료: 0.1%

💱 DeFi 프로토콜:
  활성 풀: 12개
  총 유동성: $45,250,000
  24h 거래량: $8,750,000
  스왑 수수료: 0.3%

⚙️ 시스템:
  실행 시간: 15일 7시간 23분
  메모리 사용량: 245MB
  처리된 총 트랜잭션: 12,847건
  평균 응답 시간: 1.2초

✅ 모든 시스템이 정상 작동 중입니다!
```

### Bitcoin 금고 관리

```bash
# BTC 예치
cargo run -- vault deposit --amount 100000000  # 1 BTC

# 출금 트리거 (144블록 타임락)
cargo run -- vault trigger --destination "bc1q..." --amount 50000000

# 출금 완료
cargo run -- vault complete

# 출금 취소  
cargo run -- vault cancel
```

### DeFi 작업

```bash
# 토큰 스왑
cargo run -- defi swap --from wbtc --to usdc --amount 25000000

# 풀 정보 조회
cargo run -- defi pool --token-a wbtc --token-b usdc
```

### 크로스체인 브릿지

```bash
# BTC → Solana bBTC
cargo run -- bridge lock --to-chain solana --amount 50000000 --recipient "SoL1N4x..."
```

## 🔧 개발자 가이드

### 새로운 체인 추가

1. `shared/src/types.rs`에 새 ChainId 추가
2. `shared/src/bridge.rs`에 브릿지 구현
3. `cli/src/config.rs`에 설정 추가

### 새로운 DeFi 프리미티브 추가

1. `shared/src/types.rs`에 Operation 추가
2. `mini-rollup/src/batch.rs`에 검증 로직 추가
3. `cli/src/commands/defi.rs`에 CLI 명령어 추가

### 테스트 실행

```bash
# 단위 테스트
cargo test

# 통합 테스트
cargo test --test integration

# 벤치마크
cargo bench
```

## 📊 성능 지표

- **배치 처리**: 30초마다 최대 1,000개 작업
- **상태 검증**: BitVMX RISC-V 증명
- **크로스체인**: ~2-6블록 확인 시간
- **수수료**: 스왑 0.3%, 브릿지 0.1%

## 🛡️ 보안 고려사항

### Bitcoin L1 보안
- OP_CAT 커버넌트로 조건부 출금만 허용
- 멀티시그 + 타임락 조합
- BitVMX 검증자 네트워크

### 롤업 보안
- RISC-V ELF 프로그램 검증
- 상태 루트 체인 무결성
- 배치 처리 원자성

### 브릿지 보안
- 다중 검증자 시스템
- 슬래싱 메커니즘
- 점진적 출금 한도

## 🗺️ 로드맵

### Phase 1 (현재 - 완료 ✅)
- [x] 기본 금고 및 롤업 구현
- [x] Solana 브릿지 프로토타입
- [x] CLI 도구 개발
- [x] **실제 작동하는 데모 완성**
- [x] 통합 워크스페이스 아키텍처
- [x] 실시간 시스템 모니터링

### Phase 2 (진행 중)
- [ ] 메인넷 배포 준비
- [ ] Ethereum 브릿지 추가
- [ ] 고급 DeFi 프리미티브 (유동성 공급, 대출)
- [ ] 성능 최적화 및 보안 감사

### Phase 3 (계획)
- [ ] 거버넌스 토큰 및 DAO
- [ ] 추가 체인 지원 (Polygon, Arbitrum)
- [ ] 프론트엔드 웹 인터페이스
- [ ] 생태계 파트너십 확장

## 🤝 기여하기

1. Fork 후 feature 브랜치 생성
2. 코드 변경 및 테스트 추가
3. PR 생성 (코드 리뷰 필수)

자세한 내용은 [CONTRIBUTING.md](CONTRIBUTING.md) 참조

## 📄 라이선스

MIT License - 자세한 내용은 [LICENSE](LICENSE) 참조

## 🔗 링크

- [BitVMX 백서](https://bitvmx.org/)
- [BIP-347 (OP_CAT)](https://github.com/bitcoin/bips/blob/master/bip-0347.mediawiki)
- [개발자 문서](./docs/)
- [API 레퍼런스](./docs/api.md)

---

**Made with 🐱 by the Purrfect Team**
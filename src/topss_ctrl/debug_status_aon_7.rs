#[doc = "Register `DEBUG_STATUS_AON_7` reader"]
pub type R = crate::R<DebugStatusAon7Spec>;
#[doc = "Register `DEBUG_STATUS_AON_7` writer"]
pub type W = crate::W<DebugStatusAon7Spec>;
#[doc = "Field `app_pd_iso` reader - 0:0\\]
status reg for app_pd_iso"]
pub type AppPdIsoR = crate::BitReader;
#[doc = "Field `app_pd_iso` writer - 0:0\\]
status reg for app_pd_iso"]
pub type AppPdIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_iso_ram` reader - 1:1\\]
status reg for app_pd_iso_ram"]
pub type AppPdIsoRamR = crate::BitReader;
#[doc = "Field `app_pd_iso_ram` writer - 1:1\\]
status reg for app_pd_iso_ram"]
pub type AppPdIsoRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_isoscan` reader - 2:2\\]
status reg for app_pd_isoscan"]
pub type AppPdIsoscanR = crate::BitReader;
#[doc = "Field `app_pd_isoscan` writer - 2:2\\]
status reg for app_pd_isoscan"]
pub type AppPdIsoscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_ponin` reader - 3:3\\]
status reg for app_pd_ponin"]
pub type AppPdPoninR = crate::BitReader;
#[doc = "Field `app_pd_ponin` writer - 3:3\\]
status reg for app_pd_ponin"]
pub type AppPdPoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_pgoodin` reader - 4:4\\]
status reg for app_pd_pgoodin"]
pub type AppPdPgoodinR = crate::BitReader;
#[doc = "Field `app_pd_pgoodin` writer - 4:4\\]
status reg for app_pd_pgoodin"]
pub type AppPdPgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_ponout` reader - 5:5\\]
status reg for app_pd_ponout"]
pub type AppPdPonoutR = crate::BitReader;
#[doc = "Field `app_pd_ponout` writer - 5:5\\]
status reg for app_pd_ponout"]
pub type AppPdPonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `app_pd_pgoodout` reader - 6:6\\]
status reg for app_pd_pgoodout"]
pub type AppPdPgoodoutR = crate::BitReader;
#[doc = "Field `app_pd_pgoodout` writer - 6:6\\]
status reg for app_pd_pgoodout"]
pub type AppPdPgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_iso` reader - 7:7\\]
status reg for fec_pd_iso"]
pub type FecPdIsoR = crate::BitReader;
#[doc = "Field `fec_pd_iso` writer - 7:7\\]
status reg for fec_pd_iso"]
pub type FecPdIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_iso_ram` reader - 8:8\\]
status reg for fec_pd_iso_ram"]
pub type FecPdIsoRamR = crate::BitReader;
#[doc = "Field `fec_pd_iso_ram` writer - 8:8\\]
status reg for fec_pd_iso_ram"]
pub type FecPdIsoRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_isoscan` reader - 9:9\\]
status reg for fec_pd_isoscan"]
pub type FecPdIsoscanR = crate::BitReader;
#[doc = "Field `fec_pd_isoscan` writer - 9:9\\]
status reg for fec_pd_isoscan"]
pub type FecPdIsoscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_ponin` reader - 10:10\\]
status reg for fec_pd_ponin"]
pub type FecPdPoninR = crate::BitReader;
#[doc = "Field `fec_pd_ponin` writer - 10:10\\]
status reg for fec_pd_ponin"]
pub type FecPdPoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_pgoodin` reader - 11:11\\]
status reg for fec_pd_pgoodin"]
pub type FecPdPgoodinR = crate::BitReader;
#[doc = "Field `fec_pd_pgoodin` writer - 11:11\\]
status reg for fec_pd_pgoodin"]
pub type FecPdPgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_ponout` reader - 12:12\\]
status reg for fec_pd_ponout"]
pub type FecPdPonoutR = crate::BitReader;
#[doc = "Field `fec_pd_ponout` writer - 12:12\\]
status reg for fec_pd_ponout"]
pub type FecPdPonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fec_pd_pgoodout` reader - 13:13\\]
status reg for fec_pd_pgoodout"]
pub type FecPdPgoodoutR = crate::BitReader;
#[doc = "Field `fec_pd_pgoodout` writer - 13:13\\]
status reg for fec_pd_pgoodout"]
pub type FecPdPgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_iso` reader - 14:14\\]
status reg for hwa_pd_iso"]
pub type HwaPdIsoR = crate::BitReader;
#[doc = "Field `hwa_pd_iso` writer - 14:14\\]
status reg for hwa_pd_iso"]
pub type HwaPdIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_iso_ram` reader - 15:15\\]
status reg for hwa_pd_iso_ram"]
pub type HwaPdIsoRamR = crate::BitReader;
#[doc = "Field `hwa_pd_iso_ram` writer - 15:15\\]
status reg for hwa_pd_iso_ram"]
pub type HwaPdIsoRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_isoscan` reader - 16:16\\]
status reg for hwa_pd_isoscan"]
pub type HwaPdIsoscanR = crate::BitReader;
#[doc = "Field `hwa_pd_isoscan` writer - 16:16\\]
status reg for hwa_pd_isoscan"]
pub type HwaPdIsoscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_ponin` reader - 17:17\\]
status reg for hwa_pd_ponin"]
pub type HwaPdPoninR = crate::BitReader;
#[doc = "Field `hwa_pd_ponin` writer - 17:17\\]
status reg for hwa_pd_ponin"]
pub type HwaPdPoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_pgoodin` reader - 18:18\\]
status reg for hwa_pd_pgoodin"]
pub type HwaPdPgoodinR = crate::BitReader;
#[doc = "Field `hwa_pd_pgoodin` writer - 18:18\\]
status reg for hwa_pd_pgoodin"]
pub type HwaPdPgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_ponout` reader - 19:19\\]
status reg for hwa_pd_ponout"]
pub type HwaPdPonoutR = crate::BitReader;
#[doc = "Field `hwa_pd_ponout` writer - 19:19\\]
status reg for hwa_pd_ponout"]
pub type HwaPdPonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hwa_pd_pgoodout` reader - 20:20\\]
status reg for hwa_pd_pgoodout"]
pub type HwaPdPgoodoutR = crate::BitReader;
#[doc = "Field `hwa_pd_pgoodout` writer - 20:20\\]
status reg for hwa_pd_pgoodout"]
pub type HwaPdPgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_iso` reader - 21:21\\]
status reg for test_dbg_pd_iso"]
pub type TestDbgPdIsoR = crate::BitReader;
#[doc = "Field `test_dbg_pd_iso` writer - 21:21\\]
status reg for test_dbg_pd_iso"]
pub type TestDbgPdIsoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_iso_ram` reader - 22:22\\]
status reg for test_dbg_pd_iso_ram"]
pub type TestDbgPdIsoRamR = crate::BitReader;
#[doc = "Field `test_dbg_pd_iso_ram` writer - 22:22\\]
status reg for test_dbg_pd_iso_ram"]
pub type TestDbgPdIsoRamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_isoscan` reader - 23:23\\]
status reg for test_dbg_pd_isoscan"]
pub type TestDbgPdIsoscanR = crate::BitReader;
#[doc = "Field `test_dbg_pd_isoscan` writer - 23:23\\]
status reg for test_dbg_pd_isoscan"]
pub type TestDbgPdIsoscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_ponin` reader - 24:24\\]
status reg for test_dbg_pd_ponin"]
pub type TestDbgPdPoninR = crate::BitReader;
#[doc = "Field `test_dbg_pd_ponin` writer - 24:24\\]
status reg for test_dbg_pd_ponin"]
pub type TestDbgPdPoninW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_pgoodin` reader - 25:25\\]
status reg for test_dbg_pd_pgoodin"]
pub type TestDbgPdPgoodinR = crate::BitReader;
#[doc = "Field `test_dbg_pd_pgoodin` writer - 25:25\\]
status reg for test_dbg_pd_pgoodin"]
pub type TestDbgPdPgoodinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_ponout` reader - 26:26\\]
status reg for test_dbg_pd_ponout"]
pub type TestDbgPdPonoutR = crate::BitReader;
#[doc = "Field `test_dbg_pd_ponout` writer - 26:26\\]
status reg for test_dbg_pd_ponout"]
pub type TestDbgPdPonoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `test_dbg_pd_pgoodout` reader - 27:27\\]
status reg for test_dbg_pd_pgoodout"]
pub type TestDbgPdPgoodoutR = crate::BitReader;
#[doc = "Field `test_dbg_pd_pgoodout` writer - 27:27\\]
status reg for test_dbg_pd_pgoodout"]
pub type TestDbgPdPgoodoutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_iso"]
    #[inline(always)]
    pub fn app_pd_iso(&self) -> AppPdIsoR {
        AppPdIsoR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for app_pd_iso_ram"]
    #[inline(always)]
    pub fn app_pd_iso_ram(&self) -> AppPdIsoRamR {
        AppPdIsoRamR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for app_pd_isoscan"]
    #[inline(always)]
    pub fn app_pd_isoscan(&self) -> AppPdIsoscanR {
        AppPdIsoscanR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for app_pd_ponin"]
    #[inline(always)]
    pub fn app_pd_ponin(&self) -> AppPdPoninR {
        AppPdPoninR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_pgoodin"]
    #[inline(always)]
    pub fn app_pd_pgoodin(&self) -> AppPdPgoodinR {
        AppPdPgoodinR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for app_pd_ponout"]
    #[inline(always)]
    pub fn app_pd_ponout(&self) -> AppPdPonoutR {
        AppPdPonoutR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for app_pd_pgoodout"]
    #[inline(always)]
    pub fn app_pd_pgoodout(&self) -> AppPdPgoodoutR {
        AppPdPgoodoutR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for fec_pd_iso"]
    #[inline(always)]
    pub fn fec_pd_iso(&self) -> FecPdIsoR {
        FecPdIsoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for fec_pd_iso_ram"]
    #[inline(always)]
    pub fn fec_pd_iso_ram(&self) -> FecPdIsoRamR {
        FecPdIsoRamR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for fec_pd_isoscan"]
    #[inline(always)]
    pub fn fec_pd_isoscan(&self) -> FecPdIsoscanR {
        FecPdIsoscanR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for fec_pd_ponin"]
    #[inline(always)]
    pub fn fec_pd_ponin(&self) -> FecPdPoninR {
        FecPdPoninR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for fec_pd_pgoodin"]
    #[inline(always)]
    pub fn fec_pd_pgoodin(&self) -> FecPdPgoodinR {
        FecPdPgoodinR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for fec_pd_ponout"]
    #[inline(always)]
    pub fn fec_pd_ponout(&self) -> FecPdPonoutR {
        FecPdPonoutR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for fec_pd_pgoodout"]
    #[inline(always)]
    pub fn fec_pd_pgoodout(&self) -> FecPdPgoodoutR {
        FecPdPgoodoutR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 14:14\\]
status reg for hwa_pd_iso"]
    #[inline(always)]
    pub fn hwa_pd_iso(&self) -> HwaPdIsoR {
        HwaPdIsoR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 15:15\\]
status reg for hwa_pd_iso_ram"]
    #[inline(always)]
    pub fn hwa_pd_iso_ram(&self) -> HwaPdIsoRamR {
        HwaPdIsoRamR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 16:16\\]
status reg for hwa_pd_isoscan"]
    #[inline(always)]
    pub fn hwa_pd_isoscan(&self) -> HwaPdIsoscanR {
        HwaPdIsoscanR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
status reg for hwa_pd_ponin"]
    #[inline(always)]
    pub fn hwa_pd_ponin(&self) -> HwaPdPoninR {
        HwaPdPoninR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
status reg for hwa_pd_pgoodin"]
    #[inline(always)]
    pub fn hwa_pd_pgoodin(&self) -> HwaPdPgoodinR {
        HwaPdPgoodinR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
status reg for hwa_pd_ponout"]
    #[inline(always)]
    pub fn hwa_pd_ponout(&self) -> HwaPdPonoutR {
        HwaPdPonoutR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
status reg for hwa_pd_pgoodout"]
    #[inline(always)]
    pub fn hwa_pd_pgoodout(&self) -> HwaPdPgoodoutR {
        HwaPdPgoodoutR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
status reg for test_dbg_pd_iso"]
    #[inline(always)]
    pub fn test_dbg_pd_iso(&self) -> TestDbgPdIsoR {
        TestDbgPdIsoR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
status reg for test_dbg_pd_iso_ram"]
    #[inline(always)]
    pub fn test_dbg_pd_iso_ram(&self) -> TestDbgPdIsoRamR {
        TestDbgPdIsoRamR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
status reg for test_dbg_pd_isoscan"]
    #[inline(always)]
    pub fn test_dbg_pd_isoscan(&self) -> TestDbgPdIsoscanR {
        TestDbgPdIsoscanR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 24:24\\]
status reg for test_dbg_pd_ponin"]
    #[inline(always)]
    pub fn test_dbg_pd_ponin(&self) -> TestDbgPdPoninR {
        TestDbgPdPoninR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 25:25\\]
status reg for test_dbg_pd_pgoodin"]
    #[inline(always)]
    pub fn test_dbg_pd_pgoodin(&self) -> TestDbgPdPgoodinR {
        TestDbgPdPgoodinR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 26:26\\]
status reg for test_dbg_pd_ponout"]
    #[inline(always)]
    pub fn test_dbg_pd_ponout(&self) -> TestDbgPdPonoutR {
        TestDbgPdPonoutR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 27:27\\]
status reg for test_dbg_pd_pgoodout"]
    #[inline(always)]
    pub fn test_dbg_pd_pgoodout(&self) -> TestDbgPdPgoodoutR {
        TestDbgPdPgoodoutR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
status reg for app_pd_iso"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_iso(&mut self) -> AppPdIsoW<DebugStatusAon7Spec> {
        AppPdIsoW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
status reg for app_pd_iso_ram"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_iso_ram(&mut self) -> AppPdIsoRamW<DebugStatusAon7Spec> {
        AppPdIsoRamW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
status reg for app_pd_isoscan"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_isoscan(&mut self) -> AppPdIsoscanW<DebugStatusAon7Spec> {
        AppPdIsoscanW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
status reg for app_pd_ponin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_ponin(&mut self) -> AppPdPoninW<DebugStatusAon7Spec> {
        AppPdPoninW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
status reg for app_pd_pgoodin"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_pgoodin(&mut self) -> AppPdPgoodinW<DebugStatusAon7Spec> {
        AppPdPgoodinW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
status reg for app_pd_ponout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_ponout(&mut self) -> AppPdPonoutW<DebugStatusAon7Spec> {
        AppPdPonoutW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
status reg for app_pd_pgoodout"]
    #[inline(always)]
    #[must_use]
    pub fn app_pd_pgoodout(&mut self) -> AppPdPgoodoutW<DebugStatusAon7Spec> {
        AppPdPgoodoutW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
status reg for fec_pd_iso"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_iso(&mut self) -> FecPdIsoW<DebugStatusAon7Spec> {
        FecPdIsoW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
status reg for fec_pd_iso_ram"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_iso_ram(&mut self) -> FecPdIsoRamW<DebugStatusAon7Spec> {
        FecPdIsoRamW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
status reg for fec_pd_isoscan"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_isoscan(&mut self) -> FecPdIsoscanW<DebugStatusAon7Spec> {
        FecPdIsoscanW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
status reg for fec_pd_ponin"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_ponin(&mut self) -> FecPdPoninW<DebugStatusAon7Spec> {
        FecPdPoninW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
status reg for fec_pd_pgoodin"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_pgoodin(&mut self) -> FecPdPgoodinW<DebugStatusAon7Spec> {
        FecPdPgoodinW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
status reg for fec_pd_ponout"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_ponout(&mut self) -> FecPdPonoutW<DebugStatusAon7Spec> {
        FecPdPonoutW::new(self, 12)
    }
    #[doc = "Bit 13 - 13:13\\]
status reg for fec_pd_pgoodout"]
    #[inline(always)]
    #[must_use]
    pub fn fec_pd_pgoodout(&mut self) -> FecPdPgoodoutW<DebugStatusAon7Spec> {
        FecPdPgoodoutW::new(self, 13)
    }
    #[doc = "Bit 14 - 14:14\\]
status reg for hwa_pd_iso"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_iso(&mut self) -> HwaPdIsoW<DebugStatusAon7Spec> {
        HwaPdIsoW::new(self, 14)
    }
    #[doc = "Bit 15 - 15:15\\]
status reg for hwa_pd_iso_ram"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_iso_ram(&mut self) -> HwaPdIsoRamW<DebugStatusAon7Spec> {
        HwaPdIsoRamW::new(self, 15)
    }
    #[doc = "Bit 16 - 16:16\\]
status reg for hwa_pd_isoscan"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_isoscan(&mut self) -> HwaPdIsoscanW<DebugStatusAon7Spec> {
        HwaPdIsoscanW::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
status reg for hwa_pd_ponin"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_ponin(&mut self) -> HwaPdPoninW<DebugStatusAon7Spec> {
        HwaPdPoninW::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
status reg for hwa_pd_pgoodin"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_pgoodin(&mut self) -> HwaPdPgoodinW<DebugStatusAon7Spec> {
        HwaPdPgoodinW::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
status reg for hwa_pd_ponout"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_ponout(&mut self) -> HwaPdPonoutW<DebugStatusAon7Spec> {
        HwaPdPonoutW::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
status reg for hwa_pd_pgoodout"]
    #[inline(always)]
    #[must_use]
    pub fn hwa_pd_pgoodout(&mut self) -> HwaPdPgoodoutW<DebugStatusAon7Spec> {
        HwaPdPgoodoutW::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
status reg for test_dbg_pd_iso"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_iso(&mut self) -> TestDbgPdIsoW<DebugStatusAon7Spec> {
        TestDbgPdIsoW::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
status reg for test_dbg_pd_iso_ram"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_iso_ram(&mut self) -> TestDbgPdIsoRamW<DebugStatusAon7Spec> {
        TestDbgPdIsoRamW::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
status reg for test_dbg_pd_isoscan"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_isoscan(&mut self) -> TestDbgPdIsoscanW<DebugStatusAon7Spec> {
        TestDbgPdIsoscanW::new(self, 23)
    }
    #[doc = "Bit 24 - 24:24\\]
status reg for test_dbg_pd_ponin"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_ponin(&mut self) -> TestDbgPdPoninW<DebugStatusAon7Spec> {
        TestDbgPdPoninW::new(self, 24)
    }
    #[doc = "Bit 25 - 25:25\\]
status reg for test_dbg_pd_pgoodin"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_pgoodin(&mut self) -> TestDbgPdPgoodinW<DebugStatusAon7Spec> {
        TestDbgPdPgoodinW::new(self, 25)
    }
    #[doc = "Bit 26 - 26:26\\]
status reg for test_dbg_pd_ponout"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_ponout(&mut self) -> TestDbgPdPonoutW<DebugStatusAon7Spec> {
        TestDbgPdPonoutW::new(self, 26)
    }
    #[doc = "Bit 27 - 27:27\\]
status reg for test_dbg_pd_pgoodout"]
    #[inline(always)]
    #[must_use]
    pub fn test_dbg_pd_pgoodout(&mut self) -> TestDbgPdPgoodoutW<DebugStatusAon7Spec> {
        TestDbgPdPgoodoutW::new(self, 27)
    }
}
#[doc = "DEBUG_STATUS_AON_7\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_status_aon_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_status_aon_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugStatusAon7Spec;
impl crate::RegisterSpec for DebugStatusAon7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_status_aon_7::R`](R) reader structure"]
impl crate::Readable for DebugStatusAon7Spec {}
#[doc = "`write(|w| ..)` method takes [`debug_status_aon_7::W`](W) writer structure"]
impl crate::Writable for DebugStatusAon7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS_AON_7 to value 0"]
impl crate::Resettable for DebugStatusAon7Spec {
    const RESET_VALUE: u32 = 0;
}

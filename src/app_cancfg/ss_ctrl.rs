#[doc = "Register `SS_CTRL` reader"]
pub type R = crate::R<SsCtrlSpec>;
#[doc = "Register `SS_CTRL` writer"]
pub type W = crate::W<SsCtrlSpec>;
#[doc = "Field `NU` reader - 2:0\\]
Reserved"]
pub type NuR = crate::FieldReader;
#[doc = "Field `NU` writer - 2:0\\]
Reserved"]
pub type NuW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBGSUSP_FREE` reader - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
pub type DbgsuspFreeR = crate::BitReader;
#[doc = "Field `DBGSUSP_FREE` writer - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
pub type DbgsuspFreeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUPREGEN` reader - 4:4\\]
Wakeup Request Enable"]
pub type WakeupregenR = crate::BitReader;
#[doc = "Field `WAKEUPREGEN` writer - 4:4\\]
Wakeup Request Enable"]
pub type WakeupregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOWAKEUP` reader - 5:5\\]
Automatic Wakeup Enable"]
pub type AutowakeupR = crate::BitReader;
#[doc = "Field `AUTOWAKEUP` writer - 5:5\\]
Automatic Wakeup Enable"]
pub type AutowakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_TS_CNTR_EN` reader - 6:6\\]
External TimeStamp Counter Enable"]
pub type ExtTsCntrEnR = crate::BitReader;
#[doc = "Field `EXT_TS_CNTR_EN` writer - 6:6\\]
External TimeStamp Counter Enable"]
pub type ExtTsCntrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU0` reader - 31:7\\]
Reserved"]
pub type Nu0R = crate::FieldReader<u32>;
#[doc = "Field `NU0` writer - 31:7\\]
Reserved"]
pub type Nu0W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu(&self) -> NuR {
        NuR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
    #[inline(always)]
    pub fn dbgsusp_free(&self) -> DbgsuspFreeR {
        DbgsuspFreeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Wakeup Request Enable"]
    #[inline(always)]
    pub fn wakeupregen(&self) -> WakeupregenR {
        WakeupregenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Automatic Wakeup Enable"]
    #[inline(always)]
    pub fn autowakeup(&self) -> AutowakeupR {
        AutowakeupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
External TimeStamp Counter Enable"]
    #[inline(always)]
    pub fn ext_ts_cntr_en(&self) -> ExtTsCntrEnR {
        ExtTsCntrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    pub fn nu0(&self) -> Nu0R {
        Nu0R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu(&mut self) -> NuW<SsCtrlSpec> {
        NuW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
0-Honor Debug Suspend, 1-Disregard debug suspend"]
    #[inline(always)]
    #[must_use]
    pub fn dbgsusp_free(&mut self) -> DbgsuspFreeW<SsCtrlSpec> {
        DbgsuspFreeW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Wakeup Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wakeupregen(&mut self) -> WakeupregenW<SsCtrlSpec> {
        WakeupregenW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Automatic Wakeup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn autowakeup(&mut self) -> AutowakeupW<SsCtrlSpec> {
        AutowakeupW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
External TimeStamp Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext_ts_cntr_en(&mut self) -> ExtTsCntrEnW<SsCtrlSpec> {
        ExtTsCntrEnW::new(self, 6)
    }
    #[doc = "Bits 7:31 - 31:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu0(&mut self) -> Nu0W<SsCtrlSpec> {
        Nu0W::new(self, 7)
    }
}
#[doc = "SS_CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsCtrlSpec;
impl crate::RegisterSpec for SsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_ctrl::R`](R) reader structure"]
impl crate::Readable for SsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_ctrl::W`](W) writer structure"]
impl crate::Writable for SsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_CTRL to value 0"]
impl crate::Resettable for SsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}

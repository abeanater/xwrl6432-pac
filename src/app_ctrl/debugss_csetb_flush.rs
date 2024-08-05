#[doc = "Register `DEBUGSS_CSETB_FLUSH` reader"]
pub type R = crate::R<DebugssCsetbFlushSpec>;
#[doc = "Register `DEBUGSS_CSETB_FLUSH` writer"]
pub type W = crate::W<DebugssCsetbFlushSpec>;
#[doc = "Field `CSETB_FLUSHIN` reader - 0:0\\]
RESERVED"]
pub type CsetbFlushinR = crate::BitReader;
#[doc = "Field `CSETB_FLUSHIN` writer - 0:0\\]
RESERVED"]
pub type CsetbFlushinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSETB_FLUSHINACK` reader - 8:8\\]
RESERVED"]
pub type CsetbFlushinackR = crate::BitReader;
#[doc = "Field `CSETB_FLUSHINACK` writer - 8:8\\]
RESERVED"]
pub type CsetbFlushinackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSETB_ACQ_COMPLETE` reader - 9:9\\]
RESERVED"]
pub type CsetbAcqCompleteR = crate::BitReader;
#[doc = "Field `CSETB_ACQ_COMPLETE` writer - 9:9\\]
RESERVED"]
pub type CsetbAcqCompleteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSETB_FULL` reader - 10:10\\]
RESERVED"]
pub type CsetbFullR = crate::BitReader;
#[doc = "Field `CSETB_FULL` writer - 10:10\\]
RESERVED"]
pub type CsetbFullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    pub fn csetb_flushin(&self) -> CsetbFlushinR {
        CsetbFlushinR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED"]
    #[inline(always)]
    pub fn csetb_flushinack(&self) -> CsetbFlushinackR {
        CsetbFlushinackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
RESERVED"]
    #[inline(always)]
    pub fn csetb_acq_complete(&self) -> CsetbAcqCompleteR {
        CsetbAcqCompleteR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
RESERVED"]
    #[inline(always)]
    pub fn csetb_full(&self) -> CsetbFullR {
        CsetbFullR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn csetb_flushin(&mut self) -> CsetbFlushinW<DebugssCsetbFlushSpec> {
        CsetbFlushinW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn csetb_flushinack(&mut self) -> CsetbFlushinackW<DebugssCsetbFlushSpec> {
        CsetbFlushinackW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn csetb_acq_complete(&mut self) -> CsetbAcqCompleteW<DebugssCsetbFlushSpec> {
        CsetbAcqCompleteW::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
RESERVED"]
    #[inline(always)]
    #[must_use]
    pub fn csetb_full(&mut self) -> CsetbFullW<DebugssCsetbFlushSpec> {
        CsetbFullW::new(self, 10)
    }
}
#[doc = "DEBUGSS_CSETB_FLUSH\n\nYou can [`read`](crate::Reg::read) this register and get [`debugss_csetb_flush::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debugss_csetb_flush::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DebugssCsetbFlushSpec;
impl crate::RegisterSpec for DebugssCsetbFlushSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debugss_csetb_flush::R`](R) reader structure"]
impl crate::Readable for DebugssCsetbFlushSpec {}
#[doc = "`write(|w| ..)` method takes [`debugss_csetb_flush::W`](W) writer structure"]
impl crate::Writable for DebugssCsetbFlushSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUGSS_CSETB_FLUSH to value 0"]
impl crate::Resettable for DebugssCsetbFlushSpec {
    const RESET_VALUE: u32 = 0;
}

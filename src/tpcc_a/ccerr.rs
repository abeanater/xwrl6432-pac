#[doc = "Register `CCERR` reader"]
pub type R = crate::R<CcerrSpec>;
#[doc = "Register `CCERR` writer"]
pub type W = crate::W<CcerrSpec>;
#[doc = "Field `QTHRXCD0` reader - 0:0\\]
Queue Threshold Error for Q0: QTHRXCD0 = 0 : Watermark/threshold has not been exceeded. QTHRXCD0 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD0 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd0R = crate::BitReader;
#[doc = "Field `QTHRXCD0` writer - 0:0\\]
Queue Threshold Error for Q0: QTHRXCD0 = 0 : Watermark/threshold has not been exceeded. QTHRXCD0 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD0 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD1` reader - 1:1\\]
Queue Threshold Error for Q1: QTHRXCD1 = 0 : Watermark/threshold has not been exceeded. QTHRXCD1 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD1 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd1R = crate::BitReader;
#[doc = "Field `QTHRXCD1` writer - 1:1\\]
Queue Threshold Error for Q1: QTHRXCD1 = 0 : Watermark/threshold has not been exceeded. QTHRXCD1 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD1 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD2` reader - 2:2\\]
Queue Threshold Error for Q2: QTHRXCD2 = 0 : Watermark/threshold has not been exceeded. QTHRXCD2 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD2 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd2R = crate::BitReader;
#[doc = "Field `QTHRXCD2` writer - 2:2\\]
Queue Threshold Error for Q2: QTHRXCD2 = 0 : Watermark/threshold has not been exceeded. QTHRXCD2 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD2 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD3` reader - 3:3\\]
Queue Threshold Error for Q3: QTHRXCD3 = 0 : Watermark/threshold has not been exceeded. QTHRXCD3 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD3 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd3R = crate::BitReader;
#[doc = "Field `QTHRXCD3` writer - 3:3\\]
Queue Threshold Error for Q3: QTHRXCD3 = 0 : Watermark/threshold has not been exceeded. QTHRXCD3 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD3 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD4` reader - 4:4\\]
Queue Threshold Error for Q4: QTHRXCD4 = 0 : Watermark/threshold has not been exceeded. QTHRXCD4 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD4 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd4R = crate::BitReader;
#[doc = "Field `QTHRXCD4` writer - 4:4\\]
Queue Threshold Error for Q4: QTHRXCD4 = 0 : Watermark/threshold has not been exceeded. QTHRXCD4 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD4 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD5` reader - 5:5\\]
Queue Threshold Error for Q5: QTHRXCD5 = 0 : Watermark/threshold has not been exceeded. QTHRXCD5 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD5 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd5R = crate::BitReader;
#[doc = "Field `QTHRXCD5` writer - 5:5\\]
Queue Threshold Error for Q5: QTHRXCD5 = 0 : Watermark/threshold has not been exceeded. QTHRXCD5 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD5 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD6` reader - 6:6\\]
Queue Threshold Error for Q6: QTHRXCD6 = 0 : Watermark/threshold has not been exceeded. QTHRXCD6 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD6 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd6R = crate::BitReader;
#[doc = "Field `QTHRXCD6` writer - 6:6\\]
Queue Threshold Error for Q6: QTHRXCD6 = 0 : Watermark/threshold has not been exceeded. QTHRXCD6 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD6 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD7` reader - 7:7\\]
Queue Threshold Error for Q7: QTHRXCD7 = 0 : Watermark/threshold has not been exceeded. QTHRXCD7 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD7 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd7R = crate::BitReader;
#[doc = "Field `QTHRXCD7` writer - 7:7\\]
Queue Threshold Error for Q7: QTHRXCD7 = 0 : Watermark/threshold has not been exceeded. QTHRXCD7 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD7 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
pub type Qthrxcd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES34` reader - 15:8\\]
RESERVE FIELD"]
pub type Res34R = crate::FieldReader;
#[doc = "Field `RES34` writer - 15:8\\]
RESERVE FIELD"]
pub type Res34W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCERR` reader - 16:16\\]
Transfer Completion Code Error: TCCERR = 0 : Total number of allowed TCCs outstanding has not been reached. TCCERR = 1 : Total number of allowed TCCs has been reached. TCCERR can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors were previously clear) then an error will be signaled with TPCC error interrupt."]
pub type TcerrR = crate::BitReader;
#[doc = "Field `TCERR` writer - 16:16\\]
Transfer Completion Code Error: TCCERR = 0 : Total number of allowed TCCs outstanding has not been reached. TCCERR = 1 : Total number of allowed TCCs has been reached. TCCERR can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors were previously clear) then an error will be signaled with TPCC error interrupt."]
pub type TcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES33` reader - 31:17\\]
RESERVE FIELD"]
pub type Res33R = crate::FieldReader<u16>;
#[doc = "Field `RES33` writer - 31:17\\]
RESERVE FIELD"]
pub type Res33W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Queue Threshold Error for Q0: QTHRXCD0 = 0 : Watermark/threshold has not been exceeded. QTHRXCD0 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD0 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd0(&self) -> Qthrxcd0R {
        Qthrxcd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Queue Threshold Error for Q1: QTHRXCD1 = 0 : Watermark/threshold has not been exceeded. QTHRXCD1 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD1 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd1(&self) -> Qthrxcd1R {
        Qthrxcd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Queue Threshold Error for Q2: QTHRXCD2 = 0 : Watermark/threshold has not been exceeded. QTHRXCD2 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD2 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd2(&self) -> Qthrxcd2R {
        Qthrxcd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Queue Threshold Error for Q3: QTHRXCD3 = 0 : Watermark/threshold has not been exceeded. QTHRXCD3 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD3 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd3(&self) -> Qthrxcd3R {
        Qthrxcd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Queue Threshold Error for Q4: QTHRXCD4 = 0 : Watermark/threshold has not been exceeded. QTHRXCD4 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD4 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd4(&self) -> Qthrxcd4R {
        Qthrxcd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Queue Threshold Error for Q5: QTHRXCD5 = 0 : Watermark/threshold has not been exceeded. QTHRXCD5 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD5 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd5(&self) -> Qthrxcd5R {
        Qthrxcd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Queue Threshold Error for Q6: QTHRXCD6 = 0 : Watermark/threshold has not been exceeded. QTHRXCD6 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD6 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd6(&self) -> Qthrxcd6R {
        Qthrxcd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Queue Threshold Error for Q7: QTHRXCD7 = 0 : Watermark/threshold has not been exceeded. QTHRXCD7 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD7 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    pub fn qthrxcd7(&self) -> Qthrxcd7R {
        Qthrxcd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res34(&self) -> Res34R {
        Res34R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Transfer Completion Code Error: TCCERR = 0 : Total number of allowed TCCs outstanding has not been reached. TCCERR = 1 : Total number of allowed TCCs has been reached. TCCERR can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors were previously clear) then an error will be signaled with TPCC error interrupt."]
    #[inline(always)]
    pub fn tcerr(&self) -> TcerrR {
        TcerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res33(&self) -> Res33R {
        Res33R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Queue Threshold Error for Q0: QTHRXCD0 = 0 : Watermark/threshold has not been exceeded. QTHRXCD0 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD0 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd0(&mut self) -> Qthrxcd0W<CcerrSpec> {
        Qthrxcd0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Queue Threshold Error for Q1: QTHRXCD1 = 0 : Watermark/threshold has not been exceeded. QTHRXCD1 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD1 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd1(&mut self) -> Qthrxcd1W<CcerrSpec> {
        Qthrxcd1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Queue Threshold Error for Q2: QTHRXCD2 = 0 : Watermark/threshold has not been exceeded. QTHRXCD2 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD2 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd2(&mut self) -> Qthrxcd2W<CcerrSpec> {
        Qthrxcd2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Queue Threshold Error for Q3: QTHRXCD3 = 0 : Watermark/threshold has not been exceeded. QTHRXCD3 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD3 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd3(&mut self) -> Qthrxcd3W<CcerrSpec> {
        Qthrxcd3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Queue Threshold Error for Q4: QTHRXCD4 = 0 : Watermark/threshold has not been exceeded. QTHRXCD4 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD4 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd4(&mut self) -> Qthrxcd4W<CcerrSpec> {
        Qthrxcd4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Queue Threshold Error for Q5: QTHRXCD5 = 0 : Watermark/threshold has not been exceeded. QTHRXCD5 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD5 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd5(&mut self) -> Qthrxcd5W<CcerrSpec> {
        Qthrxcd5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Queue Threshold Error for Q6: QTHRXCD6 = 0 : Watermark/threshold has not been exceeded. QTHRXCD6 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD6 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd6(&mut self) -> Qthrxcd6W<CcerrSpec> {
        Qthrxcd6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Queue Threshold Error for Q7: QTHRXCD7 = 0 : Watermark/threshold has not been exceeded. QTHRXCD7 = 1 : Watermark/threshold has been exceeded. CCERR.QTHRXCD7 can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors (including EMR/QEMR) were previously clear) then an error will be signaled with the TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd7(&mut self) -> Qthrxcd7W<CcerrSpec> {
        Qthrxcd7W::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res34(&mut self) -> Res34W<CcerrSpec> {
        Res34W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Transfer Completion Code Error: TCCERR = 0 : Total number of allowed TCCs outstanding has not been reached. TCCERR = 1 : Total number of allowed TCCs has been reached. TCCERR can be cleared by writing a '1' to corresponding bit in CCERRCLR register. If any bit in the CCERR register is set (and all errors were previously clear) then an error will be signaled with TPCC error interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tcerr(&mut self) -> TcerrW<CcerrSpec> {
        TcerrW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res33(&mut self) -> Res33W<CcerrSpec> {
        Res33W::new(self, 17)
    }
}
#[doc = "CC Error Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccerr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccerr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerrSpec;
impl crate::RegisterSpec for CcerrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccerr::R`](R) reader structure"]
impl crate::Readable for CcerrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccerr::W`](W) writer structure"]
impl crate::Writable for CcerrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCERR to value 0"]
impl crate::Resettable for CcerrSpec {
    const RESET_VALUE: u32 = 0;
}

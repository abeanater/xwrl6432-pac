#[doc = "Register `CCSTAT` reader"]
pub type R = crate::R<CcstatSpec>;
#[doc = "Register `CCSTAT` writer"]
pub type W = crate::W<CcstatSpec>;
#[doc = "Field `EVTACTV` reader - 0:0\\]
DMA Event Active: EVTACTV = 0 : No enabled DMA Events are active within the CC. EVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
pub type EvtactvR = crate::BitReader;
#[doc = "Field `EVTACTV` writer - 0:0\\]
DMA Event Active: EVTACTV = 0 : No enabled DMA Events are active within the CC. EVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
pub type EvtactvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QEVTACTV` reader - 1:1\\]
QDMA Event Active: QEVTACTV = 0 : No enabled QDMA Events are active within the CC. QEVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
pub type QevtactvR = crate::BitReader;
#[doc = "Field `QEVTACTV` writer - 1:1\\]
QDMA Event Active: QEVTACTV = 0 : No enabled QDMA Events are active within the CC. QEVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
pub type QevtactvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACTV` reader - 2:2\\]
Transfer Request Active: TRACTV = 0 : Transfer Request processing/submission logic is inactive. TRACTV = 1 : Transfer Request processing/submission logic is active."]
pub type TractvR = crate::BitReader;
#[doc = "Field `TRACTV` writer - 2:2\\]
Transfer Request Active: TRACTV = 0 : Transfer Request processing/submission logic is inactive. TRACTV = 1 : Transfer Request processing/submission logic is active."]
pub type TractvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES64` reader - 3:3\\]
RESERVE FIELD"]
pub type Res64R = crate::BitReader;
#[doc = "Field `RES64` writer - 3:3\\]
RESERVE FIELD"]
pub type Res64W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTV` reader - 4:4\\]
Channel Controller Active: Channel Controller Active is a logical-OR of each of the ACTV signals. The ACTV bit must remain high through the life of a TR. ACTV = 0 : Channel is idle. ACTV = 1 : Channel is busy."]
pub type ActvR = crate::BitReader;
#[doc = "Field `ACTV` writer - 4:4\\]
Channel Controller Active: Channel Controller Active is a logical-OR of each of the ACTV signals. The ACTV bit must remain high through the life of a TR. ACTV = 0 : Channel is idle. ACTV = 1 : Channel is busy."]
pub type ActvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES63` reader - 7:5\\]
RESERVE FIELD"]
pub type Res63R = crate::FieldReader;
#[doc = "Field `RES63` writer - 7:5\\]
RESERVE FIELD"]
pub type Res63W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `COMPACTV` reader - 13:8\\]
Completion Request Active: Counter that tracks the total number of completion requests submitted to the TC. The counter increments when a TR is submitted with TCINTEN or TCCHEN set to '1'. The counter decrements for every valid completion code received from any of the external TCs. The CC will not service new TRs if COMPACTV count is already at the limit. COMPACTV = 0 : No completion requests outstanding. COMPACTV = 1: Total of '1' completion request outstanding. ... COMPACTV = 63 : Total of 63 completion requests are outstanding. No additional TRs will be submitted until count is less than 63."]
pub type CompactvR = crate::FieldReader;
#[doc = "Field `COMPACTV` writer - 13:8\\]
Completion Request Active: Counter that tracks the total number of completion requests submitted to the TC. The counter increments when a TR is submitted with TCINTEN or TCCHEN set to '1'. The counter decrements for every valid completion code received from any of the external TCs. The CC will not service new TRs if COMPACTV count is already at the limit. COMPACTV = 0 : No completion requests outstanding. COMPACTV = 1: Total of '1' completion request outstanding. ... COMPACTV = 63 : Total of 63 completion requests are outstanding. No additional TRs will be submitted until count is less than 63."]
pub type CompactvW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RES62` reader - 15:14\\]
RESERVE FIELD"]
pub type Res62R = crate::FieldReader;
#[doc = "Field `RES62` writer - 15:14\\]
RESERVE FIELD"]
pub type Res62W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QUEACTV0` reader - 16:16\\]
Queue 0 Active QUEACTV0 = 0 : No Evts are queued in Q0. QUEACTV0 = 1 : At least one TR is queued in Q0."]
pub type Queactv0R = crate::BitReader;
#[doc = "Field `QUEACTV0` writer - 16:16\\]
Queue 0 Active QUEACTV0 = 0 : No Evts are queued in Q0. QUEACTV0 = 1 : At least one TR is queued in Q0."]
pub type Queactv0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEACTV1` reader - 17:17\\]
Queue 1 Active QUEACTV1 = 0 : No Evts are queued in Q1. QUEACTV1 = 1 : At least one TR is queued in Q1."]
pub type Queactv1R = crate::BitReader;
#[doc = "Field `QUEACTV1` writer - 17:17\\]
Queue 1 Active QUEACTV1 = 0 : No Evts are queued in Q1. QUEACTV1 = 1 : At least one TR is queued in Q1."]
pub type Queactv1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEACTV2` reader - 18:18\\]
Queue 2 Active QUEACTV2 = 0 : No Evts are queued in Q2. QUEACTV2 = 1 : At least one TR is queued in Q2."]
pub type Queactv2R = crate::BitReader;
#[doc = "Field `QUEACTV2` writer - 18:18\\]
Queue 2 Active QUEACTV2 = 0 : No Evts are queued in Q2. QUEACTV2 = 1 : At least one TR is queued in Q2."]
pub type Queactv2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEACTV3` reader - 19:19\\]
Queue 3 Active QUEACTV3 = 0 : No Evts are queued in Q3. QUEACTV3 = 1 : At least one TR is queued in Q3."]
pub type Queactv3R = crate::BitReader;
#[doc = "Field `QUEACTV3` writer - 19:19\\]
Queue 3 Active QUEACTV3 = 0 : No Evts are queued in Q3. QUEACTV3 = 1 : At least one TR is queued in Q3."]
pub type Queactv3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEACTV4` reader - 20:20\\]
Queue 4 Active QUEACTV4 = 0 : No Evts are queued in Q4. QUEACTV4 = 1 : At least one TR is queued in Q4."]
pub type Queactv4R = crate::BitReader;
#[doc = "Field `QUEACTV4` writer - 20:20\\]
Queue 4 Active QUEACTV4 = 0 : No Evts are queued in Q4. QUEACTV4 = 1 : At least one TR is queued in Q4."]
pub type Queactv4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEACTV5` reader - 21:21\\]
Queue 5 Active QUEACTV5 = 0 : No Evts are queued in Q5. QUEACTV5 = 1 : At least one TR is queued in Q5."]
pub type Queactv5R = crate::BitReader;
#[doc = "Field `QUEACTV5` writer - 21:21\\]
Queue 5 Active QUEACTV5 = 0 : No Evts are queued in Q5. QUEACTV5 = 1 : At least one TR is queued in Q5."]
pub type Queactv5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEACTV6` reader - 22:22\\]
Queue 6 Active QUEACTV6 = 0 : No Evts are queued in Q6. QUEACTV6 = 1 : At least one TR is queued in Q6."]
pub type Queactv6R = crate::BitReader;
#[doc = "Field `QUEACTV6` writer - 22:22\\]
Queue 6 Active QUEACTV6 = 0 : No Evts are queued in Q6. QUEACTV6 = 1 : At least one TR is queued in Q6."]
pub type Queactv6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUEACTV7` reader - 23:23\\]
Queue 7 Active QUEACTV7 = 0 : No Evts are queued in Q7. QUEACTV7 = 1 : At least one TR is queued in Q7."]
pub type Queactv7R = crate::BitReader;
#[doc = "Field `QUEACTV7` writer - 23:23\\]
Queue 7 Active QUEACTV7 = 0 : No Evts are queued in Q7. QUEACTV7 = 1 : At least one TR is queued in Q7."]
pub type Queactv7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES61` reader - 31:24\\]
RESERVE FIELD"]
pub type Res61R = crate::FieldReader;
#[doc = "Field `RES61` writer - 31:24\\]
RESERVE FIELD"]
pub type Res61W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
DMA Event Active: EVTACTV = 0 : No enabled DMA Events are active within the CC. EVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
    #[inline(always)]
    pub fn evtactv(&self) -> EvtactvR {
        EvtactvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
QDMA Event Active: QEVTACTV = 0 : No enabled QDMA Events are active within the CC. QEVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
    #[inline(always)]
    pub fn qevtactv(&self) -> QevtactvR {
        QevtactvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Transfer Request Active: TRACTV = 0 : Transfer Request processing/submission logic is inactive. TRACTV = 1 : Transfer Request processing/submission logic is active."]
    #[inline(always)]
    pub fn tractv(&self) -> TractvR {
        TractvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res64(&self) -> Res64R {
        Res64R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel Controller Active: Channel Controller Active is a logical-OR of each of the ACTV signals. The ACTV bit must remain high through the life of a TR. ACTV = 0 : Channel is idle. ACTV = 1 : Channel is busy."]
    #[inline(always)]
    pub fn actv(&self) -> ActvR {
        ActvR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res63(&self) -> Res63R {
        Res63R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Completion Request Active: Counter that tracks the total number of completion requests submitted to the TC. The counter increments when a TR is submitted with TCINTEN or TCCHEN set to '1'. The counter decrements for every valid completion code received from any of the external TCs. The CC will not service new TRs if COMPACTV count is already at the limit. COMPACTV = 0 : No completion requests outstanding. COMPACTV = 1: Total of '1' completion request outstanding. ... COMPACTV = 63 : Total of 63 completion requests are outstanding. No additional TRs will be submitted until count is less than 63."]
    #[inline(always)]
    pub fn compactv(&self) -> CompactvR {
        CompactvR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res62(&self) -> Res62R {
        Res62R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Queue 0 Active QUEACTV0 = 0 : No Evts are queued in Q0. QUEACTV0 = 1 : At least one TR is queued in Q0."]
    #[inline(always)]
    pub fn queactv0(&self) -> Queactv0R {
        Queactv0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 17:17\\]
Queue 1 Active QUEACTV1 = 0 : No Evts are queued in Q1. QUEACTV1 = 1 : At least one TR is queued in Q1."]
    #[inline(always)]
    pub fn queactv1(&self) -> Queactv1R {
        Queactv1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 18:18\\]
Queue 2 Active QUEACTV2 = 0 : No Evts are queued in Q2. QUEACTV2 = 1 : At least one TR is queued in Q2."]
    #[inline(always)]
    pub fn queactv2(&self) -> Queactv2R {
        Queactv2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 19:19\\]
Queue 3 Active QUEACTV3 = 0 : No Evts are queued in Q3. QUEACTV3 = 1 : At least one TR is queued in Q3."]
    #[inline(always)]
    pub fn queactv3(&self) -> Queactv3R {
        Queactv3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 20:20\\]
Queue 4 Active QUEACTV4 = 0 : No Evts are queued in Q4. QUEACTV4 = 1 : At least one TR is queued in Q4."]
    #[inline(always)]
    pub fn queactv4(&self) -> Queactv4R {
        Queactv4R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 21:21\\]
Queue 5 Active QUEACTV5 = 0 : No Evts are queued in Q5. QUEACTV5 = 1 : At least one TR is queued in Q5."]
    #[inline(always)]
    pub fn queactv5(&self) -> Queactv5R {
        Queactv5R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 22:22\\]
Queue 6 Active QUEACTV6 = 0 : No Evts are queued in Q6. QUEACTV6 = 1 : At least one TR is queued in Q6."]
    #[inline(always)]
    pub fn queactv6(&self) -> Queactv6R {
        Queactv6R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 23:23\\]
Queue 7 Active QUEACTV7 = 0 : No Evts are queued in Q7. QUEACTV7 = 1 : At least one TR is queued in Q7."]
    #[inline(always)]
    pub fn queactv7(&self) -> Queactv7R {
        Queactv7R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res61(&self) -> Res61R {
        Res61R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
DMA Event Active: EVTACTV = 0 : No enabled DMA Events are active within the CC. EVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
    #[inline(always)]
    #[must_use]
    pub fn evtactv(&mut self) -> EvtactvW<CcstatSpec> {
        EvtactvW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
QDMA Event Active: QEVTACTV = 0 : No enabled QDMA Events are active within the CC. QEVTACTV = 1 : At least one enabled DMA Event (ER &amp; EER ESR CER) is active within the CC."]
    #[inline(always)]
    #[must_use]
    pub fn qevtactv(&mut self) -> QevtactvW<CcstatSpec> {
        QevtactvW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Transfer Request Active: TRACTV = 0 : Transfer Request processing/submission logic is inactive. TRACTV = 1 : Transfer Request processing/submission logic is active."]
    #[inline(always)]
    #[must_use]
    pub fn tractv(&mut self) -> TractvW<CcstatSpec> {
        TractvW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res64(&mut self) -> Res64W<CcstatSpec> {
        Res64W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Channel Controller Active: Channel Controller Active is a logical-OR of each of the ACTV signals. The ACTV bit must remain high through the life of a TR. ACTV = 0 : Channel is idle. ACTV = 1 : Channel is busy."]
    #[inline(always)]
    #[must_use]
    pub fn actv(&mut self) -> ActvW<CcstatSpec> {
        ActvW::new(self, 4)
    }
    #[doc = "Bits 5:7 - 7:5\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res63(&mut self) -> Res63W<CcstatSpec> {
        Res63W::new(self, 5)
    }
    #[doc = "Bits 8:13 - 13:8\\]
Completion Request Active: Counter that tracks the total number of completion requests submitted to the TC. The counter increments when a TR is submitted with TCINTEN or TCCHEN set to '1'. The counter decrements for every valid completion code received from any of the external TCs. The CC will not service new TRs if COMPACTV count is already at the limit. COMPACTV = 0 : No completion requests outstanding. COMPACTV = 1: Total of '1' completion request outstanding. ... COMPACTV = 63 : Total of 63 completion requests are outstanding. No additional TRs will be submitted until count is less than 63."]
    #[inline(always)]
    #[must_use]
    pub fn compactv(&mut self) -> CompactvW<CcstatSpec> {
        CompactvW::new(self, 8)
    }
    #[doc = "Bits 14:15 - 15:14\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res62(&mut self) -> Res62W<CcstatSpec> {
        Res62W::new(self, 14)
    }
    #[doc = "Bit 16 - 16:16\\]
Queue 0 Active QUEACTV0 = 0 : No Evts are queued in Q0. QUEACTV0 = 1 : At least one TR is queued in Q0."]
    #[inline(always)]
    #[must_use]
    pub fn queactv0(&mut self) -> Queactv0W<CcstatSpec> {
        Queactv0W::new(self, 16)
    }
    #[doc = "Bit 17 - 17:17\\]
Queue 1 Active QUEACTV1 = 0 : No Evts are queued in Q1. QUEACTV1 = 1 : At least one TR is queued in Q1."]
    #[inline(always)]
    #[must_use]
    pub fn queactv1(&mut self) -> Queactv1W<CcstatSpec> {
        Queactv1W::new(self, 17)
    }
    #[doc = "Bit 18 - 18:18\\]
Queue 2 Active QUEACTV2 = 0 : No Evts are queued in Q2. QUEACTV2 = 1 : At least one TR is queued in Q2."]
    #[inline(always)]
    #[must_use]
    pub fn queactv2(&mut self) -> Queactv2W<CcstatSpec> {
        Queactv2W::new(self, 18)
    }
    #[doc = "Bit 19 - 19:19\\]
Queue 3 Active QUEACTV3 = 0 : No Evts are queued in Q3. QUEACTV3 = 1 : At least one TR is queued in Q3."]
    #[inline(always)]
    #[must_use]
    pub fn queactv3(&mut self) -> Queactv3W<CcstatSpec> {
        Queactv3W::new(self, 19)
    }
    #[doc = "Bit 20 - 20:20\\]
Queue 4 Active QUEACTV4 = 0 : No Evts are queued in Q4. QUEACTV4 = 1 : At least one TR is queued in Q4."]
    #[inline(always)]
    #[must_use]
    pub fn queactv4(&mut self) -> Queactv4W<CcstatSpec> {
        Queactv4W::new(self, 20)
    }
    #[doc = "Bit 21 - 21:21\\]
Queue 5 Active QUEACTV5 = 0 : No Evts are queued in Q5. QUEACTV5 = 1 : At least one TR is queued in Q5."]
    #[inline(always)]
    #[must_use]
    pub fn queactv5(&mut self) -> Queactv5W<CcstatSpec> {
        Queactv5W::new(self, 21)
    }
    #[doc = "Bit 22 - 22:22\\]
Queue 6 Active QUEACTV6 = 0 : No Evts are queued in Q6. QUEACTV6 = 1 : At least one TR is queued in Q6."]
    #[inline(always)]
    #[must_use]
    pub fn queactv6(&mut self) -> Queactv6W<CcstatSpec> {
        Queactv6W::new(self, 22)
    }
    #[doc = "Bit 23 - 23:23\\]
Queue 7 Active QUEACTV7 = 0 : No Evts are queued in Q7. QUEACTV7 = 1 : At least one TR is queued in Q7."]
    #[inline(always)]
    #[must_use]
    pub fn queactv7(&mut self) -> Queactv7W<CcstatSpec> {
        Queactv7W::new(self, 23)
    }
    #[doc = "Bits 24:31 - 31:24\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res61(&mut self) -> Res61W<CcstatSpec> {
        Res61W::new(self, 24)
    }
}
#[doc = "CC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcstatSpec;
impl crate::RegisterSpec for CcstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccstat::R`](R) reader structure"]
impl crate::Readable for CcstatSpec {}
#[doc = "`write(|w| ..)` method takes [`ccstat::W`](W) writer structure"]
impl crate::Writable for CcstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCSTAT to value 0"]
impl crate::Resettable for CcstatSpec {
    const RESET_VALUE: u32 = 0;
}

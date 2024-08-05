#[doc = "Register `QSTATN` reader"]
pub type R = crate::R<QstatnSpec>;
#[doc = "Register `QSTATN` writer"]
pub type W = crate::W<QstatnSpec>;
#[doc = "Field `STRTPTR` reader - 3:0\\]
Start Pointer: Represents the offset to the head entry of QueueN in units of entries. Always enabled. Legal values = 0x0 (0th entry) to 0xF (15th entry)"]
pub type StrtptrR = crate::FieldReader;
#[doc = "Field `STRTPTR` writer - 3:0\\]
Start Pointer: Represents the offset to the head entry of QueueN in units of entries. Always enabled. Legal values = 0x0 (0th entry) to 0xF (15th entry)"]
pub type StrtptrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RES58` reader - 7:4\\]
RESERVE FIELD"]
pub type Res58R = crate::FieldReader;
#[doc = "Field `RES58` writer - 7:4\\]
RESERVE FIELD"]
pub type Res58W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NUMVAL` reader - 12:8\\]
Number of Valid Entries in QueueN: Represents the total number of entries residing in the Queue Manager FIFO at a given instant. Always enabled. Legal values = 0x0 (empty) to 0x10 (full)"]
pub type NumvalR = crate::FieldReader;
#[doc = "Field `NUMVAL` writer - 12:8\\]
Number of Valid Entries in QueueN: Represents the total number of entries residing in the Queue Manager FIFO at a given instant. Always enabled. Legal values = 0x0 (empty) to 0x10 (full)"]
pub type NumvalW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RES57` reader - 15:13\\]
RESERVE FIELD"]
pub type Res57R = crate::FieldReader;
#[doc = "Field `RES57` writer - 15:13\\]
RESERVE FIELD"]
pub type Res57W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WM` reader - 20:16\\]
Watermark for Maximum Queue Usage: Watermark tracks the most entries that have been in QueueN since reset or since the last time that the watermark (WM) was cleared. QSTATn.WM is cleared via CCERR.WMCLRn bit. Legal values = 0x0 (empty) to 0x10 (full)"]
pub type WmR = crate::FieldReader;
#[doc = "Field `WM` writer - 20:16\\]
Watermark for Maximum Queue Usage: Watermark tracks the most entries that have been in QueueN since reset or since the last time that the watermark (WM) was cleared. QSTATn.WM is cleared via CCERR.WMCLRn bit. Legal values = 0x0 (empty) to 0x10 (full)"]
pub type WmW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RES56` reader - 23:21\\]
RESERVE FIELD"]
pub type Res56R = crate::FieldReader;
#[doc = "Field `RES56` writer - 23:21\\]
RESERVE FIELD"]
pub type Res56W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `THRXCD` reader - 24:24\\]
Threshold Exceeded: THRXCD = 0 : Threshold specified by QWMTHR(A B).Qn has not been exceeded. THRXCD = 1 : Threshold specified by QWMTHR(A B).Qn has been exceeded. QSTATn.THRXCD is cleared via CCERR.WMCLRn bit."]
pub type ThrxcdR = crate::BitReader;
#[doc = "Field `THRXCD` writer - 24:24\\]
Threshold Exceeded: THRXCD = 0 : Threshold specified by QWMTHR(A B).Qn has not been exceeded. THRXCD = 1 : Threshold specified by QWMTHR(A B).Qn has been exceeded. QSTATn.THRXCD is cleared via CCERR.WMCLRn bit."]
pub type ThrxcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES55` reader - 31:25\\]
RESERVE FIELD"]
pub type Res55R = crate::FieldReader;
#[doc = "Field `RES55` writer - 31:25\\]
RESERVE FIELD"]
pub type Res55W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Start Pointer: Represents the offset to the head entry of QueueN in units of entries. Always enabled. Legal values = 0x0 (0th entry) to 0xF (15th entry)"]
    #[inline(always)]
    pub fn strtptr(&self) -> StrtptrR {
        StrtptrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res58(&self) -> Res58R {
        Res58R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Number of Valid Entries in QueueN: Represents the total number of entries residing in the Queue Manager FIFO at a given instant. Always enabled. Legal values = 0x0 (empty) to 0x10 (full)"]
    #[inline(always)]
    pub fn numval(&self) -> NumvalR {
        NumvalR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res57(&self) -> Res57R {
        Res57R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Watermark for Maximum Queue Usage: Watermark tracks the most entries that have been in QueueN since reset or since the last time that the watermark (WM) was cleared. QSTATn.WM is cleared via CCERR.WMCLRn bit. Legal values = 0x0 (empty) to 0x10 (full)"]
    #[inline(always)]
    pub fn wm(&self) -> WmR {
        WmR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - 23:21\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res56(&self) -> Res56R {
        Res56R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - 24:24\\]
Threshold Exceeded: THRXCD = 0 : Threshold specified by QWMTHR(A B).Qn has not been exceeded. THRXCD = 1 : Threshold specified by QWMTHR(A B).Qn has been exceeded. QSTATn.THRXCD is cleared via CCERR.WMCLRn bit."]
    #[inline(always)]
    pub fn thrxcd(&self) -> ThrxcdR {
        ThrxcdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - 31:25\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res55(&self) -> Res55R {
        Res55R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Start Pointer: Represents the offset to the head entry of QueueN in units of entries. Always enabled. Legal values = 0x0 (0th entry) to 0xF (15th entry)"]
    #[inline(always)]
    #[must_use]
    pub fn strtptr(&mut self) -> StrtptrW<QstatnSpec> {
        StrtptrW::new(self, 0)
    }
    #[doc = "Bits 4:7 - 7:4\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res58(&mut self) -> Res58W<QstatnSpec> {
        Res58W::new(self, 4)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Number of Valid Entries in QueueN: Represents the total number of entries residing in the Queue Manager FIFO at a given instant. Always enabled. Legal values = 0x0 (empty) to 0x10 (full)"]
    #[inline(always)]
    #[must_use]
    pub fn numval(&mut self) -> NumvalW<QstatnSpec> {
        NumvalW::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res57(&mut self) -> Res57W<QstatnSpec> {
        Res57W::new(self, 13)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Watermark for Maximum Queue Usage: Watermark tracks the most entries that have been in QueueN since reset or since the last time that the watermark (WM) was cleared. QSTATn.WM is cleared via CCERR.WMCLRn bit. Legal values = 0x0 (empty) to 0x10 (full)"]
    #[inline(always)]
    #[must_use]
    pub fn wm(&mut self) -> WmW<QstatnSpec> {
        WmW::new(self, 16)
    }
    #[doc = "Bits 21:23 - 23:21\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res56(&mut self) -> Res56W<QstatnSpec> {
        Res56W::new(self, 21)
    }
    #[doc = "Bit 24 - 24:24\\]
Threshold Exceeded: THRXCD = 0 : Threshold specified by QWMTHR(A B).Qn has not been exceeded. THRXCD = 1 : Threshold specified by QWMTHR(A B).Qn has been exceeded. QSTATn.THRXCD is cleared via CCERR.WMCLRn bit."]
    #[inline(always)]
    #[must_use]
    pub fn thrxcd(&mut self) -> ThrxcdW<QstatnSpec> {
        ThrxcdW::new(self, 24)
    }
    #[doc = "Bits 25:31 - 31:25\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res55(&mut self) -> Res55W<QstatnSpec> {
        Res55W::new(self, 25)
    }
}
#[doc = "QSTATn Register Set\n\nYou can [`read`](crate::Reg::read) this register and get [`qstatn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qstatn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QstatnSpec;
impl crate::RegisterSpec for QstatnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qstatn::R`](R) reader structure"]
impl crate::Readable for QstatnSpec {}
#[doc = "`write(|w| ..)` method takes [`qstatn::W`](W) writer structure"]
impl crate::Writable for QstatnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QSTATN to value 0"]
impl crate::Resettable for QstatnSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `CCERRCLR` reader"]
pub type R = crate::R<CcerrclrSpec>;
#[doc = "Register `CCERRCLR` writer"]
pub type W = crate::W<CcerrclrSpec>;
#[doc = "Field `QTHRXCD0` reader - 0:0\\]
Clear error for CCERR.QTHRXCD0: Write of '1' clears the values of QSTAT0.WM QSTAT0.THRXCD CCERR.QTHRXCD0 Writes of '0' have no affect."]
pub type Qthrxcd0R = crate::BitReader;
#[doc = "Field `QTHRXCD0` writer - 0:0\\]
Clear error for CCERR.QTHRXCD0: Write of '1' clears the values of QSTAT0.WM QSTAT0.THRXCD CCERR.QTHRXCD0 Writes of '0' have no affect."]
pub type Qthrxcd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD1` reader - 1:1\\]
Clear error for CCERR.QTHRXCD1: Write of '1' clears the values of QSTAT1.WM QSTAT1.THRXCD CCERR.QTHRXCD1 Writes of '0' have no affect."]
pub type Qthrxcd1R = crate::BitReader;
#[doc = "Field `QTHRXCD1` writer - 1:1\\]
Clear error for CCERR.QTHRXCD1: Write of '1' clears the values of QSTAT1.WM QSTAT1.THRXCD CCERR.QTHRXCD1 Writes of '0' have no affect."]
pub type Qthrxcd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD2` reader - 2:2\\]
Clear error for CCERR.QTHRXCD2: Write of '1' clears the values of QSTAT2.WM QSTAT2.THRXCD CCERR.QTHRXCD2 Writes of '0' have no affect."]
pub type Qthrxcd2R = crate::BitReader;
#[doc = "Field `QTHRXCD2` writer - 2:2\\]
Clear error for CCERR.QTHRXCD2: Write of '1' clears the values of QSTAT2.WM QSTAT2.THRXCD CCERR.QTHRXCD2 Writes of '0' have no affect."]
pub type Qthrxcd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD3` reader - 3:3\\]
Clear error for CCERR.QTHRXCD3: Write of '1' clears the values of QSTAT3.WM QSTAT3.THRXCD CCERR.QTHRXCD3 Writes of '0' have no affect."]
pub type Qthrxcd3R = crate::BitReader;
#[doc = "Field `QTHRXCD3` writer - 3:3\\]
Clear error for CCERR.QTHRXCD3: Write of '1' clears the values of QSTAT3.WM QSTAT3.THRXCD CCERR.QTHRXCD3 Writes of '0' have no affect."]
pub type Qthrxcd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD4` reader - 4:4\\]
Clear error for CCERR.QTHRXCD4: Write of '1' clears the values of QSTAT4.WM QSTAT4.THRXCD CCERR.QTHRXCD4 Writes of '0' have no affect."]
pub type Qthrxcd4R = crate::BitReader;
#[doc = "Field `QTHRXCD4` writer - 4:4\\]
Clear error for CCERR.QTHRXCD4: Write of '1' clears the values of QSTAT4.WM QSTAT4.THRXCD CCERR.QTHRXCD4 Writes of '0' have no affect."]
pub type Qthrxcd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD5` reader - 5:5\\]
Clear error for CCERR.QTHRXCD5: Write of '1' clears the values of QSTAT5.WM QSTAT5.THRXCD CCERR.QTHRXCD5 Writes of '0' have no affect."]
pub type Qthrxcd5R = crate::BitReader;
#[doc = "Field `QTHRXCD5` writer - 5:5\\]
Clear error for CCERR.QTHRXCD5: Write of '1' clears the values of QSTAT5.WM QSTAT5.THRXCD CCERR.QTHRXCD5 Writes of '0' have no affect."]
pub type Qthrxcd5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD6` reader - 6:6\\]
Clear error for CCERR.QTHRXCD6: Write of '1' clears the values of QSTAT6.WM QSTAT6.THRXCD CCERR.QTHRXCD6 Writes of '0' have no affect."]
pub type Qthrxcd6R = crate::BitReader;
#[doc = "Field `QTHRXCD6` writer - 6:6\\]
Clear error for CCERR.QTHRXCD6: Write of '1' clears the values of QSTAT6.WM QSTAT6.THRXCD CCERR.QTHRXCD6 Writes of '0' have no affect."]
pub type Qthrxcd6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QTHRXCD7` reader - 7:7\\]
Clear error for CCERR.QTHRXCD7: Write of '1' clears the values of QSTAT7.WM QSTAT7.THRXCD CCERR.QTHRXCD7 Writes of '0' have no affect."]
pub type Qthrxcd7R = crate::BitReader;
#[doc = "Field `QTHRXCD7` writer - 7:7\\]
Clear error for CCERR.QTHRXCD7: Write of '1' clears the values of QSTAT7.WM QSTAT7.THRXCD CCERR.QTHRXCD7 Writes of '0' have no affect."]
pub type Qthrxcd7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES36` reader - 15:8\\]
RESERVE FIELD"]
pub type Res36R = crate::FieldReader;
#[doc = "Field `RES36` writer - 15:8\\]
RESERVE FIELD"]
pub type Res36W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCERR` reader - 16:16\\]
Clear Error for CCERR.TCERR: Write of '1' clears the value of CCERR bit N. Writes of '0' have no affect."]
pub type TcerrR = crate::BitReader;
#[doc = "Field `TCERR` writer - 16:16\\]
Clear Error for CCERR.TCERR: Write of '1' clears the value of CCERR bit N. Writes of '0' have no affect."]
pub type TcerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES35` reader - 31:17\\]
RESERVE FIELD"]
pub type Res35R = crate::FieldReader<u16>;
#[doc = "Field `RES35` writer - 31:17\\]
RESERVE FIELD"]
pub type Res35W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Clear error for CCERR.QTHRXCD0: Write of '1' clears the values of QSTAT0.WM QSTAT0.THRXCD CCERR.QTHRXCD0 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd0(&self) -> Qthrxcd0R {
        Qthrxcd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear error for CCERR.QTHRXCD1: Write of '1' clears the values of QSTAT1.WM QSTAT1.THRXCD CCERR.QTHRXCD1 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd1(&self) -> Qthrxcd1R {
        Qthrxcd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear error for CCERR.QTHRXCD2: Write of '1' clears the values of QSTAT2.WM QSTAT2.THRXCD CCERR.QTHRXCD2 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd2(&self) -> Qthrxcd2R {
        Qthrxcd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear error for CCERR.QTHRXCD3: Write of '1' clears the values of QSTAT3.WM QSTAT3.THRXCD CCERR.QTHRXCD3 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd3(&self) -> Qthrxcd3R {
        Qthrxcd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear error for CCERR.QTHRXCD4: Write of '1' clears the values of QSTAT4.WM QSTAT4.THRXCD CCERR.QTHRXCD4 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd4(&self) -> Qthrxcd4R {
        Qthrxcd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear error for CCERR.QTHRXCD5: Write of '1' clears the values of QSTAT5.WM QSTAT5.THRXCD CCERR.QTHRXCD5 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd5(&self) -> Qthrxcd5R {
        Qthrxcd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear error for CCERR.QTHRXCD6: Write of '1' clears the values of QSTAT6.WM QSTAT6.THRXCD CCERR.QTHRXCD6 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd6(&self) -> Qthrxcd6R {
        Qthrxcd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear error for CCERR.QTHRXCD7: Write of '1' clears the values of QSTAT7.WM QSTAT7.THRXCD CCERR.QTHRXCD7 Writes of '0' have no affect."]
    #[inline(always)]
    pub fn qthrxcd7(&self) -> Qthrxcd7R {
        Qthrxcd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res36(&self) -> Res36R {
        Res36R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear Error for CCERR.TCERR: Write of '1' clears the value of CCERR bit N. Writes of '0' have no affect."]
    #[inline(always)]
    pub fn tcerr(&self) -> TcerrR {
        TcerrR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - 31:17\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res35(&self) -> Res35R {
        Res35R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Clear error for CCERR.QTHRXCD0: Write of '1' clears the values of QSTAT0.WM QSTAT0.THRXCD CCERR.QTHRXCD0 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd0(&mut self) -> Qthrxcd0W<CcerrclrSpec> {
        Qthrxcd0W::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Clear error for CCERR.QTHRXCD1: Write of '1' clears the values of QSTAT1.WM QSTAT1.THRXCD CCERR.QTHRXCD1 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd1(&mut self) -> Qthrxcd1W<CcerrclrSpec> {
        Qthrxcd1W::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Clear error for CCERR.QTHRXCD2: Write of '1' clears the values of QSTAT2.WM QSTAT2.THRXCD CCERR.QTHRXCD2 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd2(&mut self) -> Qthrxcd2W<CcerrclrSpec> {
        Qthrxcd2W::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Clear error for CCERR.QTHRXCD3: Write of '1' clears the values of QSTAT3.WM QSTAT3.THRXCD CCERR.QTHRXCD3 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd3(&mut self) -> Qthrxcd3W<CcerrclrSpec> {
        Qthrxcd3W::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Clear error for CCERR.QTHRXCD4: Write of '1' clears the values of QSTAT4.WM QSTAT4.THRXCD CCERR.QTHRXCD4 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd4(&mut self) -> Qthrxcd4W<CcerrclrSpec> {
        Qthrxcd4W::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Clear error for CCERR.QTHRXCD5: Write of '1' clears the values of QSTAT5.WM QSTAT5.THRXCD CCERR.QTHRXCD5 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd5(&mut self) -> Qthrxcd5W<CcerrclrSpec> {
        Qthrxcd5W::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Clear error for CCERR.QTHRXCD6: Write of '1' clears the values of QSTAT6.WM QSTAT6.THRXCD CCERR.QTHRXCD6 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd6(&mut self) -> Qthrxcd6W<CcerrclrSpec> {
        Qthrxcd6W::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
Clear error for CCERR.QTHRXCD7: Write of '1' clears the values of QSTAT7.WM QSTAT7.THRXCD CCERR.QTHRXCD7 Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn qthrxcd7(&mut self) -> Qthrxcd7W<CcerrclrSpec> {
        Qthrxcd7W::new(self, 7)
    }
    #[doc = "Bits 8:15 - 15:8\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res36(&mut self) -> Res36W<CcerrclrSpec> {
        Res36W::new(self, 8)
    }
    #[doc = "Bit 16 - 16:16\\]
Clear Error for CCERR.TCERR: Write of '1' clears the value of CCERR bit N. Writes of '0' have no affect."]
    #[inline(always)]
    #[must_use]
    pub fn tcerr(&mut self) -> TcerrW<CcerrclrSpec> {
        TcerrW::new(self, 16)
    }
    #[doc = "Bits 17:31 - 31:17\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res35(&mut self) -> Res35W<CcerrclrSpec> {
        Res35W::new(self, 17)
    }
}
#[doc = "CC Error Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccerrclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccerrclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcerrclrSpec;
impl crate::RegisterSpec for CcerrclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccerrclr::R`](R) reader structure"]
impl crate::Readable for CcerrclrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccerrclr::W`](W) writer structure"]
impl crate::Writable for CcerrclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCERRCLR to value 0"]
impl crate::Resettable for CcerrclrSpec {
    const RESET_VALUE: u32 = 0;
}

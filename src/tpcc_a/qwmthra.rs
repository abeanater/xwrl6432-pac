#[doc = "Register `QWMTHRA` reader"]
pub type R = crate::R<QwmthraSpec>;
#[doc = "Register `QWMTHRA` writer"]
pub type W = crate::W<QwmthraSpec>;
#[doc = "Field `Q0` reader - 4:0\\]
Queue Threshold for Q0 value"]
pub type Q0R = crate::FieldReader;
#[doc = "Field `Q0` writer - 4:0\\]
Queue Threshold for Q0 value"]
pub type Q0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RES60` reader - 7:5\\]
RESERVE FIELD"]
pub type Res60R = crate::FieldReader;
#[doc = "Field `RES60` writer - 7:5\\]
RESERVE FIELD"]
pub type Res60W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Q1` reader - 12:8\\]
Queue Threshold for Q1 value"]
pub type Q1R = crate::FieldReader;
#[doc = "Field `Q1` writer - 12:8\\]
Queue Threshold for Q1 value"]
pub type Q1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RES59` reader - 31:13\\]
RESERVE FIELD"]
pub type Res59R = crate::FieldReader<u32>;
#[doc = "Field `RES59` writer - 31:13\\]
RESERVE FIELD"]
pub type Res59W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:4 - 4:0\\]
Queue Threshold for Q0 value"]
    #[inline(always)]
    pub fn q0(&self) -> Q0R {
        Q0R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - 7:5\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res60(&self) -> Res60R {
        Res60R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Queue Threshold for Q1 value"]
    #[inline(always)]
    pub fn q1(&self) -> Q1R {
        Q1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res59(&self) -> Res59R {
        Res59R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - 4:0\\]
Queue Threshold for Q0 value"]
    #[inline(always)]
    #[must_use]
    pub fn q0(&mut self) -> Q0W<QwmthraSpec> {
        Q0W::new(self, 0)
    }
    #[doc = "Bits 5:7 - 7:5\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res60(&mut self) -> Res60W<QwmthraSpec> {
        Res60W::new(self, 5)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Queue Threshold for Q1 value"]
    #[inline(always)]
    #[must_use]
    pub fn q1(&mut self) -> Q1W<QwmthraSpec> {
        Q1W::new(self, 8)
    }
    #[doc = "Bits 13:31 - 31:13\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res59(&mut self) -> Res59W<QwmthraSpec> {
        Res59W::new(self, 13)
    }
}
#[doc = "Queue Threshold A for Q\\[3:0\\]: CCERR.QTHRXCDn and QSTATn.THRXCD error bit is set when the number of Events in QueueN at an instant in time (visible via QSTATn.NUMVAL) equals or exceeds the value specified by QWMTHRA.Qn. Legal values = 0x0 (ever used?) to 0x10 (ever full?) A value of 0x11 disables threshold errors.\n\nYou can [`read`](crate::Reg::read) this register and get [`qwmthra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qwmthra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QwmthraSpec;
impl crate::RegisterSpec for QwmthraSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qwmthra::R`](R) reader structure"]
impl crate::Readable for QwmthraSpec {}
#[doc = "`write(|w| ..)` method takes [`qwmthra::W`](W) writer structure"]
impl crate::Writable for QwmthraSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QWMTHRA to value 0"]
impl crate::Resettable for QwmthraSpec {
    const RESET_VALUE: u32 = 0;
}

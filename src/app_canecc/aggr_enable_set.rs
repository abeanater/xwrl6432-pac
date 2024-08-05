#[doc = "Register `AGGR_ENABLE_SET` reader"]
pub type R = crate::R<AggrEnableSetSpec>;
#[doc = "Register `AGGR_ENABLE_SET` writer"]
pub type W = crate::W<AggrEnableSetSpec>;
#[doc = "Field `PARITY` reader - 0:0\\]
Interrupt Enable Set Register for parity errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - 0:0\\]
Interrupt Enable Set Register for parity errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - 1:1\\]
Interrupt Enable Set Register for svbus timeout errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 1:1\\]
Interrupt Enable Set Register for svbus timeout errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU15` reader - 31:2\\]
Reserved"]
pub type Nu15R = crate::FieldReader<u32>;
#[doc = "Field `NU15` writer - 31:2\\]
Reserved"]
pub type Nu15W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for parity errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for svbus timeout errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn nu15(&self) -> Nu15R {
        Nu15R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Enable Set Register for parity errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<AggrEnableSetSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Enable Set Register for svbus timeout errors. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field."]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<AggrEnableSetSpec> {
        TimeoutW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu15(&mut self) -> Nu15W<AggrEnableSetSpec> {
        Nu15W::new(self, 2)
    }
}
#[doc = "AGGR interrupt enable set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrEnableSetSpec;
impl crate::RegisterSpec for AggrEnableSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_enable_set::R`](R) reader structure"]
impl crate::Readable for AggrEnableSetSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_enable_set::W`](W) writer structure"]
impl crate::Writable for AggrEnableSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGGR_ENABLE_SET to value 0"]
impl crate::Resettable for AggrEnableSetSpec {
    const RESET_VALUE: u32 = 0;
}

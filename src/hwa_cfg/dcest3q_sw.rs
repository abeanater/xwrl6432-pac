#[doc = "Register `DCEST3Q_SW` reader"]
pub type R = crate::R<Dcest3qSwSpec>;
#[doc = "Register `DCEST3Q_SW` writer"]
pub type W = crate::W<Dcest3qSwSpec>;
#[doc = "Field `DCEST3Q_SW` reader - 23:0\\]
This register holds the software programmed dc value Q to be subtracted from incoming sample for bcnt = 2."]
pub type Dcest3qSwR = crate::FieldReader<u32>;
#[doc = "Field `DCEST3Q_SW` writer - 23:0\\]
This register holds the software programmed dc value Q to be subtracted from incoming sample for bcnt = 2."]
pub type Dcest3qSwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the software programmed dc value Q to be subtracted from incoming sample for bcnt = 2."]
    #[inline(always)]
    pub fn dcest3q_sw(&self) -> Dcest3qSwR {
        Dcest3qSwR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the software programmed dc value Q to be subtracted from incoming sample for bcnt = 2."]
    #[inline(always)]
    #[must_use]
    pub fn dcest3q_sw(&mut self) -> Dcest3qSwW<Dcest3qSwSpec> {
        Dcest3qSwW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest3qSwSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST3Q_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest3q_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest3q_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest3qSwSpec;
impl crate::RegisterSpec for Dcest3qSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest3q_sw::R`](R) reader structure"]
impl crate::Readable for Dcest3qSwSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest3q_sw::W`](W) writer structure"]
impl crate::Writable for Dcest3qSwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST3Q_SW to value 0"]
impl crate::Resettable for Dcest3qSwSpec {
    const RESET_VALUE: u32 = 0;
}
#[doc = "Register `DC_ACC5Q_MSB` reader"]
pub type R = crate::R<DcAcc5qMsbSpec>;
#[doc = "Register `DC_ACC5Q_MSB` writer"]
pub type W = crate::W<DcAcc5qMsbSpec>;
#[doc = "Field `DC_ACC5Q_MSB` reader - 3:0\\]
This register provides the MSB 4 bits value of DC accumulator Q channel for bcnt=4"]
pub type DcAcc5qMsbR = crate::FieldReader;
#[doc = "Field `DC_ACC5Q_MSB` writer - 3:0\\]
This register provides the MSB 4 bits value of DC accumulator Q channel for bcnt=4"]
pub type DcAcc5qMsbW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
This register provides the MSB 4 bits value of DC accumulator Q channel for bcnt=4"]
    #[inline(always)]
    pub fn dc_acc5q_msb(&self) -> DcAcc5qMsbR {
        DcAcc5qMsbR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
This register provides the MSB 4 bits value of DC accumulator Q channel for bcnt=4"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc5q_msb(&mut self) -> DcAcc5qMsbW<DcAcc5qMsbSpec> {
        DcAcc5qMsbW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<DcAcc5qMsbSpec> {
        Nu1W::new(self, 4)
    }
}
#[doc = "DC_ACC5Q_MSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc5q_msb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc5q_msb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAcc5qMsbSpec;
impl crate::RegisterSpec for DcAcc5qMsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc5q_msb::R`](R) reader structure"]
impl crate::Readable for DcAcc5qMsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc5q_msb::W`](W) writer structure"]
impl crate::Writable for DcAcc5qMsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC5Q_MSB to value 0"]
impl crate::Resettable for DcAcc5qMsbSpec {
    const RESET_VALUE: u32 = 0;
}

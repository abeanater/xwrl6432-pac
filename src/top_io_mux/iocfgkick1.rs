#[doc = "Register `IOCFGKICK1` reader"]
pub type R = crate::R<Iocfgkick1Spec>;
#[doc = "Register `IOCFGKICK1` writer"]
pub type W = crate::W<Iocfgkick1Spec>;
#[doc = "Field `IOCFGKICK1` reader - 31:0\\]
Kicker 1 Register. The value 95A4 F1E0h must be written to the KICK1 as part of the process to unlock the CPU write access to above PINMUX registers (excluding IOCFGKICK0). IOCFGKICK0 has to be written with 83E70B13h to enable access to IOCFGKICK1."]
pub type Iocfgkick1R = crate::FieldReader<u32>;
#[doc = "Field `IOCFGKICK1` writer - 31:0\\]
Kicker 1 Register. The value 95A4 F1E0h must be written to the KICK1 as part of the process to unlock the CPU write access to above PINMUX registers (excluding IOCFGKICK0). IOCFGKICK0 has to be written with 83E70B13h to enable access to IOCFGKICK1."]
pub type Iocfgkick1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Kicker 1 Register. The value 95A4 F1E0h must be written to the KICK1 as part of the process to unlock the CPU write access to above PINMUX registers (excluding IOCFGKICK0). IOCFGKICK0 has to be written with 83E70B13h to enable access to IOCFGKICK1."]
    #[inline(always)]
    pub fn iocfgkick1(&self) -> Iocfgkick1R {
        Iocfgkick1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Kicker 1 Register. The value 95A4 F1E0h must be written to the KICK1 as part of the process to unlock the CPU write access to above PINMUX registers (excluding IOCFGKICK0). IOCFGKICK0 has to be written with 83E70B13h to enable access to IOCFGKICK1."]
    #[inline(always)]
    #[must_use]
    pub fn iocfgkick1(&mut self) -> Iocfgkick1W<Iocfgkick1Spec> {
        Iocfgkick1W::new(self, 0)
    }
}
#[doc = "IOCFGKICK1\n\nYou can [`read`](crate::Reg::read) this register and get [`iocfgkick1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iocfgkick1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Iocfgkick1Spec;
impl crate::RegisterSpec for Iocfgkick1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iocfgkick1::R`](R) reader structure"]
impl crate::Readable for Iocfgkick1Spec {}
#[doc = "`write(|w| ..)` method takes [`iocfgkick1::W`](W) writer structure"]
impl crate::Writable for Iocfgkick1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IOCFGKICK1 to value 0"]
impl crate::Resettable for Iocfgkick1Spec {
    const RESET_VALUE: u32 = 0;
}
